use clap::Args;
use thiserror::Error;

#[derive(Debug, Args)]
pub struct CompletionsArgs {}

#[derive(Debug, Error)]
pub enum CompletionsError {}

impl CompletionsArgs {
    pub(crate) fn run(&self) -> Result<String, CompletionsError> {
        todo!()
    }
}
