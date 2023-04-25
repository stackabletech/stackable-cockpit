use anyhow::Result;
use clap::Parser;
use dotenvy::dotenv;
use tracing::metadata::LevelFilter;
use tracing_subscriber::fmt;

use crate::cli::Commands;

pub mod cli;
pub mod cmds;
pub mod constants;

#[tokio::main]
async fn main() -> Result<()> {
    // Parse the CLI args and commands
    let cli = cli::Cli::parse();

    // Catch if --offline is used for now
    if cli.offline {
        todo!()
    }

    // Construct the tracing subscriber
    let format = fmt::format()
        .with_level(false)
        .with_ansi(true)
        .without_time();

    tracing_subscriber::fmt()
        .with_max_level(match cli.log_level {
            Some(level) => LevelFilter::from_level(level),
            None => LevelFilter::OFF,
        })
        .event_format(format)
        .pretty()
        .init();

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
        Commands::Cache(args) => args.run()?,
    };

    println!("{output}");
    Ok(())
}
