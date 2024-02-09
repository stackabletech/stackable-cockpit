use std::pin::pin;

use clap::Args;
use rand::Rng;
use snafu::{ResultExt, Snafu};
use stackable_operator::{
    k8s_openapi::api::core::v1::{ContainerStatus, EphemeralContainer, Pod, PodSpec},
    kube::{
        self,
        api::{AttachParams, PatchParams},
    },
};
use termion::raw::IntoRawMode;
use tracing::info;

use crate::cli::Cli;

#[derive(Debug, Snafu)]
pub enum CmdError {
    Attach { source: kube::Error },
}

#[derive(Debug, Args)]
pub struct DebugArgs {
    #[clap(long, short)]
    namespace: String,
    pod: String,
    #[clap(long)]
    image: String,
}

impl DebugArgs {
    pub async fn run(&self, _cli: &Cli) -> Result<String, CmdError> {
        let kube = kube::Client::try_default().await.unwrap();
        let pods = kube::Api::<Pod>::namespaced(kube, &self.namespace);
        let mut rng = rand::thread_rng();
        let mut debug_container_name = "sble-debug-".to_string();
        for _ in 0..5 {
            debug_container_name.push(rng.gen_range('a'..='z'));
        }
        info!(
            container.name = debug_container_name,
            "Creating debug container"
        );
        let pod = Pod {
            spec: Some(PodSpec {
                ephemeral_containers: Some(vec![EphemeralContainer {
                    name: debug_container_name.clone(),
                    image: Some(self.image.clone()),
                    tty: Some(true),
                    stdin: Some(true),
                    ..Default::default()
                }]),
                ..Default::default()
            }),
            ..Default::default()
        };
        pods.patch_ephemeral_containers(
            &self.pod,
            &PatchParams::default(),
            &kube::api::Patch::Strategic(pod),
        )
        .await
        .unwrap();
        info!(
            container.name = debug_container_name,
            "Waiting for container to start"
        );
        let ready_pod =
            kube::runtime::wait::await_condition(pods.clone(), &self.pod, |pod: Option<&Pod>| {
                pod.and_then(debug_container_status_of_pod(&debug_container_name))
                    .and_then(|c| Some(c.state.as_ref()?.waiting.is_none()))
                    .unwrap_or_default()
            })
            .await
            .unwrap();
        dbg!(ready_pod
            .as_ref()
            .and_then(debug_container_status_of_pod(&debug_container_name)));
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
            futures::future::select(
                pin!(tokio::io::copy(
                    &mut attachment.stdout().unwrap(),
                    &mut tokio::io::stdout()
                )),
                pin!(tokio::io::copy(
                    &mut tokio::io::stdin(),
                    &mut attachment.stdin().unwrap()
                )),
            )
            .await
            .factor_first()
            .0
            .unwrap();
        }
        // FIXME: Terminate the process to avoid Tokio hogging stdin forever
        std::process::exit(0);
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
