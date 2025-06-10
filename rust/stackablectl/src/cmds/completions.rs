use clap::{Args, CommandFactory, Subcommand};
use clap_complete::{
    Generator,
    Shell::{Bash, Elvish, Fish, Zsh},
    generate,
};
use clap_complete_nushell::Nushell;
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

    /// Generate shell completions for Elvish
    Elvish,

    /// Generate shell completions for Fish
    Fish,

    /// Generate shell completions for Nushell
    Nushell,

    /// Generate shell completions for ZSH
    Zsh,
}

#[derive(Debug, Snafu)]
pub enum CmdError {
    #[snafu(display("failed to convert completion output into string"))]
    StringConvert { source: std::string::FromUtf8Error },
}

impl CompletionsArgs {
    pub fn run(&self) -> Result<String, CmdError> {
        match &self.subcommand {
            CompletionCommands::Bash => generate_completions(Bash),
            CompletionCommands::Fish => generate_completions(Fish),
            CompletionCommands::Elvish => generate_completions(Elvish),
            CompletionCommands::Nushell => generate_completions(Nushell),
            CompletionCommands::Zsh => generate_completions(Zsh),
        }
    }
}

fn generate_completions<G>(shell: G) -> Result<String, CmdError>
where
    G: Generator,
{
    let mut cmd = Cli::command();
    let mut buf = Vec::new();

    generate(shell, &mut cmd, "stackablectl", &mut buf);
    String::from_utf8(buf).context(StringConvertSnafu)
}
