use clap::Args;
use snafu::{ResultExt, Snafu};
use tokio::sync::mpsc;

use crate::cli::Cli;

use self::{
    input_handling::handle_event,
    logging::init_logging,
    message::Message,
    rendering::render,
    state::{update, update_stacklets_loop, Model, RunningState},
};

mod input_handling;
mod logging;
mod message;
mod rendering;
mod state;

#[derive(Debug, Snafu)]
// #[snafu(module)]
pub enum CmdError {
    #[snafu(display("failed to draw to terminal"))]
    DrawToTerminal { source: std::io::Error },

    #[snafu(display("failed to handle user input"))]
    InputHandling { source: input_handling::Error },
}

#[derive(Debug, Args)]
pub struct ControlCenterArgs {}

impl ControlCenterArgs {
    pub async fn run(&self, _cli: &Cli) -> Result<(), CmdError> {
        init_logging();

        let result = self.inner_run().await;
        ratatui::restore();
        result
    }

    pub async fn inner_run(&self) -> Result<(), CmdError> {
        let mut terminal = ratatui::init();

        let mut model = Model::default();
        let (message_tx, mut message_rx) = mpsc::channel::<Message>(10);

        let message_tx_for_loop = message_tx.clone();
        tokio::spawn(async { update_stacklets_loop(message_tx_for_loop).await });

        while model.running_state != RunningState::Done {
            // Render the current view
            terminal
                .draw(|frame| render(&mut model, frame))
                .context(DrawToTerminalSnafu)?;

            // Handle events and map to a Message
            handle_event(&model, message_tx.clone())
                .await
                .context(InputHandlingSnafu)?;

            while let Ok(message) = message_rx.try_recv() {
                update(&mut model, message);
            }
        }

        Ok(())
    }
}
