use std::{cmp::min, time::Duration};

use ratatui::widgets::TableState;
use snafu::{ResultExt, Snafu};
use stackable_cockpit::{
    platform::stacklet::{self, list_stacklets, Stacklet},
    utils::k8s::{self, Client},
};
use tokio::{sync::mpsc::Sender, time::MissedTickBehavior};
use tracing::instrument;
use tui_logger::{TuiWidgetEvent, TuiWidgetState};

use super::rendering::tabs::SelectedTab;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("failed to create Kubernetes client"))]
    KubeClientCreate { source: k8s::Error },

    #[snafu(display("failed to list stacklets"))]
    StackletList { source: stacklet::Error },
}

#[derive(Default)]
pub struct Model {
    pub running_state: RunningState,
    pub selected_tab: SelectedTab,
    pub stacklets: Vec<Stacklet>,
    pub stacklets_table_state: TableState,
    pub logging_widget_state: TuiWidgetState,
}

#[derive(Debug, Default, PartialEq)]
pub enum RunningState {
    #[default]
    Running,
    Done,
}

#[derive(Debug)]
pub enum Message {
    Quit,
    StackletListUp {
        steps: usize,
    },
    StackletListDown {
        steps: usize,
    },
    StackletListStart,
    StackletListEnd,
    StackletUpdate {
        stacklets: Vec<Stacklet>,
    },
    NextTab,
    PreviousTab,
    #[allow(clippy::enum_variant_names)]
    LoggingWidgetMessage(TuiWidgetEvent),
}

#[instrument(skip(model))]
pub fn update(model: &mut Model, message: Message) -> Option<Message> {
    match message {
        Message::Quit => model.running_state = RunningState::Done,
        Message::StackletUpdate { stacklets } => model.stacklets = stacklets,
        Message::StackletListUp { steps } => {
            let new_selected = match model.stacklets_table_state.selected() {
                Some(index) => {
                    if index == 0 {
                        model.stacklets.len().saturating_sub(1)
                    } else {
                        index.saturating_sub(steps)
                    }
                }
                None => 0,
            };
            model.stacklets_table_state.select(Some(new_selected));
        }
        Message::StackletListDown { steps } => {
            let new_selected = match model.stacklets_table_state.selected() {
                Some(index) => {
                    if index >= model.stacklets.len().saturating_sub(1) {
                        0
                    } else {
                        min(index + steps, model.stacklets.len().saturating_sub(1))
                    }
                }
                None => 0,
            };
            model.stacklets_table_state.select(Some(new_selected));
        }
        Message::StackletListStart => {
            model.stacklets_table_state.select(Some(0));
        }
        Message::StackletListEnd => {
            model
                .stacklets_table_state
                .select(Some(model.stacklets.len().saturating_sub(1)));
        }
        Message::NextTab => {
            model.selected_tab = model.selected_tab.next();
        }
        Message::PreviousTab => {
            model.selected_tab = model.selected_tab.previous();
        }
        Message::LoggingWidgetMessage(message) => model.logging_widget_state.transition(message),
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
            .await
            .context(StackletListSnafu)?;

        message_tx
            .send(Message::StackletUpdate { stacklets })
            .await
            .unwrap();

        interval.tick().await;
    }
}
