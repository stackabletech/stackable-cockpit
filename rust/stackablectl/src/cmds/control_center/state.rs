use std::time::Duration;

use ratatui::widgets::TableState;
use snafu::{ResultExt, Snafu};
use stackable_cockpit::{
    platform::stacklet::{self, list_stacklets, Stacklet},
    utils::k8s::{self, Client},
};
use tokio::{sync::mpsc::Sender, time::MissedTickBehavior};
use tracing::instrument;

use super::message::Message;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("failed to create Kubernetes client"))]
    KubeClientCreate { source: k8s::Error },

    #[snafu(display("failed to list stacklets"))]
    StackletList { source: stacklet::Error },
}

#[derive(Debug, Default)]
pub struct Model {
    pub running_state: RunningState,
    pub stacklets: Vec<Stacklet>,
    pub stacklets_table_state: TableState,
}

#[derive(Debug, Default, PartialEq)]
pub enum RunningState {
    #[default]
    Running,
    Done,
}

#[instrument]
pub fn update(model: &mut Model, message: Message) -> Option<Message> {
    match message {
        Message::StackletUpdate { stacklets } => model.stacklets = stacklets,
        Message::Quit => model.running_state = RunningState::Done,
    };

    None
}

#[instrument]
pub async fn update_stacklets_loop(message_tx: Sender<Message>) -> Result<(), Error> {
    let client = Client::new().await.context(KubeClientCreateSnafu)?;

    let mut interval = tokio::time::interval(Duration::from_secs(1));
    interval.set_missed_tick_behavior(MissedTickBehavior::Delay);

    loop {
        let stacklets = list_stacklets(&client, None /* We list them in all namespaces */)
            // list_stackable_stacklets(&client, None /* We list them in all namespaces */)
            .await
            .context(StackletListSnafu)?;
        message_tx
            .send(Message::StackletUpdate { stacklets })
            .await
            .unwrap();

        interval.tick().await;
    }
}
