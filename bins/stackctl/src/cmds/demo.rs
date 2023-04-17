use clap::Args;
use thiserror::Error;

#[derive(Debug, Args)]
pub struct DemoArgs {}

#[derive(Debug, Error)]
pub enum DemoError {}

impl DemoArgs {
    pub(crate) fn run(&self) -> Result<String, DemoError> {
        todo!()
    }
}
