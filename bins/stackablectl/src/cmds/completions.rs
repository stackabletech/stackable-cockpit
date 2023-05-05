use clap::{Args, CommandFactory, Subcommand};
use clap_complete::{generate, Shell};
use snafu::{ResultExt, Snafu};

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

#[derive(Debug, Snafu)]
pub enum CompletionsError {
    #[snafu(display("string error: {source}"))]
    StringError { source: std::string::FromUtf8Error },
}

impl CompletionsArgs {
    pub fn run(&self) -> Result<String, CompletionsError> {
        match &self.subcommand {
            CompletionCommands::Bash => generate_completions(Shell::Bash),
            CompletionCommands::Fish => generate_completions(Shell::Fish),
            CompletionCommands::Zsh => generate_completions(Shell::Zsh),
        }
    }
}

fn generate_completions(shell: Shell) -> Result<String, CompletionsError> {
    let mut cmd = Cli::command();
    let mut buf = Vec::new();

    generate(shell, &mut cmd, "stackablectl", &mut buf);
    Ok(String::from_utf8(buf).context(StringSnafu {})?)
}
