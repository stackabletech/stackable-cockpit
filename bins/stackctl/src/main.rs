use anyhow::Result;
use clap::Parser;
use dotenvy::dotenv;

use crate::cli::Commands;

pub mod cli;
pub mod cmds;
pub mod constants;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = cli::Cli::parse();

    // Load env vars from optional .env file
    match dotenv() {
        Ok(_) => {}
        Err(err) => {
            if !err.not_found() {
                return Err(err.into());
            }
        }
    }

    let output = match &cli.subcommand {
        Commands::Operator(args) => args.run()?,
        Commands::Release(args) => args.run()?,
        Commands::Stack(args) => args.run()?,
        Commands::Services(args) => args.run()?,
        Commands::Demo(args) => args.run(&cli).await?,
        Commands::Completions(args) => args.run()?,
    };

    println!("{output}");
    Ok(())
}
