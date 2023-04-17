use anyhow::Result;
use clap::Parser;

use crate::cli::Commands;

pub mod cli;
pub mod cmds;

fn main() -> Result<()> {
    let cli = cli::Cli::parse();

    let output = match cli.subcommand {
        Commands::Operator(args) => args.run()?,
        Commands::Release(args) => args.run()?,
        Commands::Stack(args) => args.run()?,
        Commands::Services(args) => args.run()?,
        Commands::Demo(args) => args.run()?,
        Commands::Completions(args) => args.run()?,
    };

    println!("{output}");
    Ok(())
}
