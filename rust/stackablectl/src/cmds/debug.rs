use std::{
    io::{Read, Stdin},
    os::fd::AsRawFd,
    task::{ready, Poll},
};

use clap::Args;
use futures::{channel::mpsc::Sender, FutureExt, SinkExt, TryFutureExt};
use rand::Rng;
use snafu::{ResultExt, Snafu};
use stackable_operator::{
    k8s_openapi::api::core::v1::{ContainerStatus, EphemeralContainer, Pod, PodSpec},
    kube::{
        self,
        api::{AttachParams, PatchParams, TerminalSize},
    },
};
use termion::{raw::IntoRawMode, terminal_size};
use tokio::{
    io::{unix::AsyncFd, AsyncRead},
    signal::unix::SignalKind,
};
use tracing::{error, info};

use crate::cli::Cli;

#[derive(Debug, Snafu)]
pub enum CmdError {
    Attach { source: kube::Error },
}

#[derive(Debug, Args)]
pub struct DebugArgs {
    #[clap(long, short)]
    namespace: Option<String>,
    pod: String,
    #[clap(long, short)]
    container: String,
    #[clap(long)]
    image: String,
    #[clap(last = true)]
    cmd: Option<Vec<String>>,
}

impl DebugArgs {
    pub async fn run(&self, _cli: &Cli) -> Result<String, CmdError> {
        let kube = kube::Client::try_default().await.unwrap();
        let pods = match &self.namespace {
            Some(ns) => kube::Api::<Pod>::namespaced(kube, ns),
            None => kube::Api::<Pod>::default_namespaced(kube),
        };
        let mut rng = rand::thread_rng();
        let mut debug_container_name = "sble-debug-".to_string();
        for _ in 0..5 {
            debug_container_name.push(rng.gen_range('a'..='z'));
        }
        info!(
            container.name = debug_container_name,
            "Creating debug container"
        );
        let pod = pods.get(&self.pod).await.unwrap();
        let template_container = pod
            .spec
            .as_ref()
            .and_then(|spec| spec.containers.iter().find(|c| c.name == self.container))
            .unwrap();
        let pod_patch = Pod {
            spec: Some(PodSpec {
                ephemeral_containers: Some(vec![EphemeralContainer {
                    name: debug_container_name.clone(),
                    image: Some(self.image.clone()),
                    tty: Some(true),
                    stdin: Some(true),

                    command: self.cmd.clone(),
                    args: self.cmd.is_some().then(Vec::new),

                    // copy environment from template
                    env: template_container.env.clone(),
                    env_from: template_container.env_from.clone(),
                    volume_mounts: template_container.volume_mounts.clone(),
                    volume_devices: template_container.volume_devices.clone(),

                    ..Default::default()
                }]),
                ..Default::default()
            }),
            ..Default::default()
        };
        pods.patch_ephemeral_containers(
            &self.pod,
            &PatchParams::default(),
            &kube::api::Patch::Strategic(pod_patch),
        )
        .await
        .unwrap();
        info!(
            container.name = debug_container_name,
            "Waiting for container to start"
        );
        let ready_pod =
            kube::runtime::wait::await_condition(pods.clone(), &self.pod, |pod: Option<&Pod>| {
                let container = pod.and_then(debug_container_status_of_pod(&debug_container_name));
                container
                    .and_then(|c| Some(c.state.as_ref()?.waiting.is_none()))
                    .unwrap_or_default()
                    || container
                        .and_then(|c| c.last_state.as_ref()?.terminated.as_ref())
                        .is_some()
            })
            .await
            .unwrap();
        let debug_container_status = ready_pod
            .as_ref()
            .and_then(debug_container_status_of_pod(&debug_container_name))
            .unwrap();
        if let Some(termination) = debug_container_status
            .last_state
            .as_ref()
            .and_then(|state| state.terminated.as_ref())
        {
            error!(
                error = termination.message,
                exit_code = termination.exit_code,
                "Debug container failed to start!"
            );
        }
        info!(
            container.name = debug_container_name,
            "Attaching to container"
        );
        let mut attachment = pods
            .attach(
                &self.pod,
                &AttachParams::interactive_tty().container(debug_container_name),
            )
            .await
            .context(AttachSnafu)?;
        info!("Attached to container, if the shell line looks empty, press ENTER!");
        {
            let _raw = std::io::stdout().into_raw_mode().unwrap();
            futures::future::select_all([
                update_terminal_size(attachment.terminal_size().unwrap())
                    .map(Ok)
                    .boxed(),
                tokio::io::copy(&mut attachment.stdout().unwrap(), &mut tokio::io::stdout())
                    .map_ok(drop)
                    .boxed(),
                tokio::io::copy(&mut AsyncStdin::new(), &mut attachment.stdin().unwrap())
                    .map_ok(drop)
                    .boxed(),
            ])
            .await
            .0
            .unwrap();
        }
        Ok(String::new())
    }
}

/// Does true non-blocking reads of stdin, so that we can cancel properly on shutdown.
/// The compromise is that it does not handle having things piped into it very well, since their write sides
/// will also be turned non-blocking.
///
/// Also, `AsyncStdin` will currently _not_ restore the mode of stdin when dropped.
// FIXME: restore status
struct AsyncStdin {
    fd: AsyncFd<Stdin>,
}

impl AsyncStdin {
    fn new() -> Self {
        let stdin = std::io::stdin();
        // Make stdin non-blocking
        {
            let old_flags = unsafe { libc::fcntl(stdin.as_raw_fd(), libc::F_GETFL) };
            if old_flags == -1 {
                panic!("{:?}", std::io::Error::last_os_error());
            }
            let status = unsafe {
                libc::fcntl(
                    stdin.as_raw_fd(),
                    libc::F_SETFL,
                    old_flags | libc::O_NONBLOCK,
                )
            };
            if status == -1 {
                panic!("{:?}", std::io::Error::last_os_error());
            }
        };
        Self {
            fd: AsyncFd::new(stdin).unwrap(),
        }
    }
}

impl AsyncRead for AsyncStdin {
    fn poll_read(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        let mut ready = ready!(self.fd.poll_read_ready_mut(cx)?);
        match ready.try_io(|r| {
            let read = r.get_mut().read(buf.initialize_unfilled())?;
            buf.advance(read);
            Ok(())
        }) {
            Ok(res) => Poll::Ready(res),
            Err(_would_block) => Poll::Pending,
        }
    }
}

async fn update_terminal_size(mut tx: Sender<TerminalSize>) {
    let mut signal = tokio::signal::unix::signal(SignalKind::window_change()).unwrap();
    {
        let (width, height) = terminal_size().unwrap();
        // Make TTY apps re-render by force-changing the terminal size
        // Start by sending an invalid size so that it's a change no matter
        // whether the size has actually changed.
        tx.send(TerminalSize {
            width: width - 1,
            height,
        })
        .await
        .unwrap();
        tx.send(TerminalSize { width, height }).await.unwrap();
    }
    while let Some(()) = signal.recv().await {
        let (width, height) = terminal_size().unwrap();
        tx.send(TerminalSize { width, height }).await.unwrap();
    }
}

fn debug_container_status_of_pod(
    debug_container_name: &str,
) -> impl for<'a> Fn(&'a Pod) -> Option<&'a ContainerStatus> + '_ {
    move |pod: &Pod| {
        pod.status
            .as_ref()?
            .ephemeral_container_statuses
            .as_ref()?
            .iter()
            .find(|c| c.name == debug_container_name)
    }
}
