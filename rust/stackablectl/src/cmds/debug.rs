use std::{
    io::{Read, Stdin},
    os::fd::AsRawFd,
    task::{Poll, ready},
};

use clap::Args;
use futures::{
    FutureExt, SinkExt, TryFutureExt,
    channel::mpsc::{self, Sender},
};
use rand::Rng;
use snafu::{OptionExt, ResultExt, Snafu, futures::TryFutureExt as _};
use stackable_operator::{
    builder::pod::security::SecurityContextBuilder,
    k8s_openapi::api::core::v1::{ContainerStatus, EphemeralContainer, Pod, PodSpec},
    kube::{
        self,
        api::{AttachParams, PatchParams, TerminalSize},
        runtime::reflector::ObjectRef,
    },
};
use termion::{raw::IntoRawMode, terminal_size};
use tokio::{
    io::{AsyncRead, unix::AsyncFd},
    signal::unix::SignalKind,
};
use tracing::{Instrument, error, info, info_span, warn};

use crate::cli::Cli;

#[derive(Debug, Snafu)]
pub enum CmdError {
    #[snafu(display("failed to create Kubernetes client"))]
    KubeClientCreate { source: kube::Error },

    #[snafu(display("failed to get {pod}"))]
    GetPod {
        source: kube::Error,
        pod: ObjectRef<Pod>,
    },

    #[snafu(display("{pod} has no container {container:?}"))]
    FindTemplateContainer {
        pod: ObjectRef<Pod>,
        container: String,
    },

    #[snafu(display("failed to create ephemeral debug container {container:?} on {pod}"))]
    CreateDebugContainer {
        source: kube::Error,
        pod: ObjectRef<Pod>,
        container: String,
    },

    #[snafu(display("debug container {container:?} on {pod} never became ready"))]
    AwaitDebugContainerReadiness {
        source: kube::runtime::wait::Error,
        pod: ObjectRef<Pod>,
        container: String,
    },

    #[snafu(display("failed to get status of debug container {container:?} on {pod}"))]
    FindDebugContainerStatus {
        pod: ObjectRef<Pod>,
        container: String,
    },

    #[snafu(display("failed to attach to container {container:?} on {pod}"))]
    AttachContainer {
        source: kube::Error,
        pod: ObjectRef<Pod>,
        container: String,
    },

    #[snafu(display("failed to enable raw local TTY input"))]
    SetRawTtyMode { source: std::io::Error },

    #[snafu(display("failed to turn stdin async"))]
    AsyncifyStdin { source: std::io::Error },

    #[snafu(display("failed to initialize AsyncFd for stdin"))]
    AsyncFdStdin { source: std::io::Error },

    #[snafu(display("container has no terminal size channel"))]
    NoTerminalSizeChannel,

    #[snafu(display("failed to read terminal size"))]
    GetTerminalSize { source: std::io::Error },

    #[snafu(display("failed to update terminal size"))]
    UpdateTerminalSize { source: mpsc::SendError },

    #[snafu(display("container has no stdin channel"))]
    NoStdinChannel,

    #[snafu(display("container has no stdout channel"))]
    NoStdoutChannel,

    #[snafu(display("failed to forward stdin to container"))]
    ForwardStdin { source: std::io::Error },

    #[snafu(display("failed to forward stdout from container"))]
    ForwardStdout { source: std::io::Error },
}
type Result<T, E = CmdError> = std::result::Result<T, E>;

#[derive(Debug, Args)]
pub struct DebugArgs {
    /// The namespace of the Pod being debugged
    #[clap(long, short)]
    namespace: Option<String>,

    /// The Pod to debug
    pod: String,

    /// The target container to debug
    ///
    /// Volumes and environment variables will be copied from this container.
    #[clap(long, short)]
    container: String,

    /// The debug container image
    ///
    /// Defaults to the image of the target container if not specified.
    #[clap(long)]
    image: Option<String>,

    /// The command to run in the debug container
    #[clap(last = true)]
    cmd: Option<Vec<String>>,
}

impl DebugArgs {
    pub async fn run(&self, _cli: &Cli) -> Result<String, CmdError> {
        let kube = kube::Client::try_default()
            .await
            .context(KubeClientCreateSnafu)?;
        let namespace = self
            .namespace
            .as_deref()
            .unwrap_or_else(|| kube.default_namespace());
        let pods = kube::Api::<Pod>::namespaced(kube.clone(), namespace);
        let debug_container_name = generate_debug_container_name();
        let span = info_span!("debug", container.debug.name = debug_container_name);
        async {
            info!("Creating debug container");
            let pod_ref = || ObjectRef::<Pod>::new(&self.pod).within(namespace);
            let pod = pods
                .get(&self.pod)
                .await
                .with_context(|_| GetPodSnafu { pod: pod_ref() })?;
            let template_container = pod
                .spec
                .as_ref()
                .and_then(|spec| spec.containers.iter().find(|c| c.name == self.container))
                .with_context(|| FindTemplateContainerSnafu {
                    pod: pod_ref(),
                    container: &self.container,
                })?;
            let pod_patch = Pod {
                spec: Some(PodSpec {
                    ephemeral_containers: Some(vec![EphemeralContainer {
                        name: debug_container_name.clone(),
                        image: self
                            .image
                            .clone()
                            .or_else(|| template_container.image.clone()),
                        tty: Some(true),
                        stdin: Some(true),

                        command: self.cmd.clone(),
                        args: self.cmd.is_some().then(Vec::new),

                        security_context: Some(SecurityContextBuilder::run_as_root()),

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
            .with_context(|_| CreateDebugContainerSnafu {
                pod: pod_ref(),
                container: &self.container,
            })?;
            info!("Waiting for container to start");
            let ready_pod = kube::runtime::wait::await_condition(
                pods.clone(),
                &self.pod,
                |pod: Option<&Pod>| {
                    let container =
                        pod.and_then(debug_container_status_of_pod(&debug_container_name));
                    container
                        .and_then(|c| Some(c.state.as_ref()?.waiting.is_none()))
                        .unwrap_or_default()
                        || container
                            .and_then(|c| c.last_state.as_ref()?.terminated.as_ref())
                            .is_some()
                },
            )
            .await
            .with_context(|_| AwaitDebugContainerReadinessSnafu {
                pod: pod_ref(),
                container: &self.container,
            })?;
            let debug_container_status = ready_pod
                .as_ref()
                .and_then(debug_container_status_of_pod(&debug_container_name))
                .with_context(|| FindDebugContainerStatusSnafu {
                    pod: pod_ref(),
                    container: &self.container,
                })?;
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
            info!("Attaching to container");
            let mut attachment = pods
                .attach(
                    &self.pod,
                    &AttachParams::interactive_tty().container(debug_container_name),
                )
                .await
                .with_context(|_| AttachContainerSnafu {
                    pod: pod_ref(),
                    container: &self.container,
                })?;
            info!("Attached to container, if the shell line looks empty, press ENTER!");
            let _raw = std::io::stdout()
                .into_raw_mode()
                .context(SetRawTtyModeSnafu)?;
            futures::future::select_all([
                update_terminal_size(
                    attachment
                        .terminal_size()
                        .context(NoTerminalSizeChannelSnafu)?,
                )
                .boxed(),
                tokio::io::copy(
                    &mut attachment.stdout().context(NoStdoutChannelSnafu)?,
                    &mut tokio::io::stdout(),
                )
                .map_ok(drop)
                .context(ForwardStdoutSnafu)
                .boxed(),
                tokio::io::copy(
                    &mut AsyncStdin::new()?,
                    &mut attachment.stdin().context(NoStdinChannelSnafu)?,
                )
                .map_ok(drop)
                .context(ForwardStdinSnafu)
                .boxed(),
            ])
            .await
            .0?;
            Ok(String::new())
        }
        .instrument(span)
        .await
    }
}

fn generate_debug_container_name() -> String {
    let mut rng = rand::thread_rng();
    let mut name = "stackablectl-debug-".to_string();
    for _ in 0..5 {
        name.push(rng.gen_range('a'..='z'));
    }
    name
}

/// Does true non-blocking reads of stdin, so that we can cancel properly on shutdown.
/// The compromise is that it does not handle having things piped into it very well, since their write sides
/// will also be turned non-blocking.
///
/// Only a single instance of AsyncStdin should exist at any one time.
///
/// Also, `AsyncStdin` will restore the mode of stdin when dropped.
struct AsyncStdin {
    fd: AsyncFd<Stdin>,
    old_flags: i32,
}

impl AsyncStdin {
    #[tracing::instrument]
    fn new() -> Result<Self> {
        let stdin = std::io::stdin();
        // Make stdin non-blocking
        let old_flags;
        {
            old_flags = unsafe { libc::fcntl(stdin.as_raw_fd(), libc::F_GETFL) };
            if old_flags == -1 {
                return Err(std::io::Error::last_os_error()).context(AsyncifyStdinSnafu);
            }
            if old_flags & libc::O_NONBLOCK != 0 {
                warn!(
                    "stdin is already non-blocking (did you try to create multiple AsyncStdin instances?)"
                );
            }
            let status = unsafe {
                libc::fcntl(
                    stdin.as_raw_fd(),
                    libc::F_SETFL,
                    old_flags | libc::O_NONBLOCK,
                )
            };
            if status == -1 {
                return Err(std::io::Error::last_os_error()).context(AsyncifyStdinSnafu);
            }
        };
        Ok(Self {
            fd: AsyncFd::new(stdin).context(AsyncFdStdinSnafu)?,
            old_flags,
        })
    }
}

impl Drop for AsyncStdin {
    fn drop(&mut self) {
        let status = unsafe { libc::fcntl(self.fd.as_raw_fd(), libc::F_SETFL, self.old_flags) };
        if status == -1 {
            panic!(
                "unable to revert stdin flags: {}",
                std::io::Error::last_os_error()
            );
        }
    }
}

impl AsyncRead for AsyncStdin {
    fn poll_read(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        loop {
            let mut ready = ready!(self.fd.poll_read_ready_mut(cx)?);

            let io = ready.try_io(|r| {
                let read = r.get_mut().read(buf.initialize_unfilled())?;
                buf.advance(read);
                Ok(())
            });

            break match io {
                Ok(res) => Poll::Ready(res),
                Err(_would_block) => {
                    // Try to poll again, so that we re-register the waker
                    continue;
                }
            };
        }
    }
}

async fn update_terminal_size(mut tx: Sender<TerminalSize>) -> Result<()> {
    let mut signal = tokio::signal::unix::signal(SignalKind::window_change()).unwrap();
    {
        let (width, height) = terminal_size().context(GetTerminalSizeSnafu)?;
        // Make TTY apps re-render by force-changing the terminal size
        // Start by sending an invalid size so that it's a change no matter
        // whether the size has actually changed.
        tx.send(TerminalSize {
            width: width - 1,
            height,
        })
        .await
        .context(UpdateTerminalSizeSnafu)?;
        tx.send(TerminalSize { width, height })
            .await
            .context(UpdateTerminalSizeSnafu)?;
    }
    while let Some(()) = signal.recv().await {
        let (width, height) = terminal_size().context(GetTerminalSizeSnafu)?;
        tx.send(TerminalSize { width, height })
            .await
            .context(UpdateTerminalSizeSnafu)?;
    }
    Ok(())
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
