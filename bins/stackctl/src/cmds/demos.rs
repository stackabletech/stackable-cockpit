use clap::Args;
use thiserror::Error;

#[derive(Debug, Args)]
pub struct DemosArgs {}

#[derive(Debug, Error)]
pub enum DemosError {}

impl DemosArgs {
    pub(crate) fn run(&self) -> Result<String, DemosError> {
        todo!()
    }
}
