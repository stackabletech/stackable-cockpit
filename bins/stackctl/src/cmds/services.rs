use clap::Args;
use thiserror::Error;

#[derive(Debug, Args)]
pub struct ServicesArgs {}

#[derive(Debug, Error)]
pub enum ServicesError {}

impl ServicesArgs {
    pub(crate) fn run(&self) -> Result<String, ServicesError> {
        todo!()
    }
}
