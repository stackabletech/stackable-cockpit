use clap::Args;
use thiserror::Error;

#[derive(Debug, Args)]
pub struct StackArgs {}

#[derive(Debug, Error)]
pub enum StackError {}

impl StackArgs {
    pub(crate) fn run(&self) -> Result<String, StackError> {
        todo!()
    }
}
