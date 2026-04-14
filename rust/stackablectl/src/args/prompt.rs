use clap::Args;

#[derive(Debug, Args)]
#[command(next_help_heading = "Prompt options")]
pub struct CommonPromptArgs {
    /// Assume "yes" as answer to all prompts and run non-interactively
    #[arg(long, visible_aliases(["yes"]), short = 'y', global = true)]
    pub assume_yes: bool,
}
