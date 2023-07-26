use clap::{Args, CommandFactory, Subcommand};
use clap_complete::{generate, Shell};
use thiserror::Error;

use crate::cli::Cli;

#[derive(Debug, Args)]
pub struct CompletionsArgs {
    #[command(subcommand)]
    subcommand: CompletionCommands,
}

#[derive(Debug, Subcommand)]
pub enum CompletionCommands {
    /// Generate shell completions for Bash
    Bash,

    /// Generate shell completions for Fish
    Fish,

    /// Generate shell completions for ZSH
    Zsh,
}

#[derive(Debug, Error)]
pub enum CompletionsCmdError {
    #[error("failed to encode expanded completions as UTF-8")]
    StringError(#[from] std::string::FromUtf8Error),
}

impl CompletionsArgs {
    pub fn run(&self) -> Result<String, CompletionsCmdError> {
        match &self.subcommand {
            CompletionCommands::Bash => generate_completions(Shell::Bash),
            CompletionCommands::Fish => generate_completions(Shell::Fish),
            CompletionCommands::Zsh => generate_completions(Shell::Zsh),
        }
    }
}

fn generate_completions(shell: Shell) -> Result<String, CompletionsCmdError> {
    let mut cmd = Cli::command();
    let mut buf = Vec::new();

    generate(shell, &mut cmd, "stackablectl", &mut buf);
    Ok(String::from_utf8(buf)?)
}
