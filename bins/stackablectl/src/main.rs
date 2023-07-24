use clap::Parser;
use dotenvy::dotenv;
use thiserror::Error;
use tracing::metadata::LevelFilter;
use tracing_subscriber::fmt;

use stackablectl::{
    cli::{self, Commands},
    cmds::{
        cache::CacheCmdError, completions::CompletionsCmdError, demo::DemoCmdError,
        operator::OperatorCmdError, release::ReleaseCmdError, stack::StackCmdError,
        stacklets::StackletsCmdError,
    },
};

#[derive(Debug, Error)]
enum CliError {
    #[error("operator command error")]
    OperatorCmdError(#[from] OperatorCmdError),

    #[error("release command error")]
    ReleaseCmdError(#[from] ReleaseCmdError),

    #[error("stack command error")]
    StackCmdError(#[from] StackCmdError),

    #[error("stacklets command error")]
    StackletsCmdError(#[from] StackletsCmdError),

    #[error("demo command error")]
    DemoCmdError(#[from] DemoCmdError),

    #[error("completions command error")]
    CompletionsCmdError(#[from] CompletionsCmdError),

    #[error("cache command error")]
    CacheCmdError(#[from] CacheCmdError),
}

#[tokio::main]
async fn main() -> Result<(), CliError> {
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
                eprintln!("{err}")
            }
        }
    }

    // Add Helm repos
    if let Err(err) = cli.add_helm_repos() {
        eprintln!("{err}")
    };

    let output = match &cli.subcommand {
        Commands::Operator(args) => args.run(&cli).await?,
        Commands::Release(args) => args.run(&cli).await?,
        Commands::Stack(args) => args.run(&cli).await?,
        Commands::Stacklets(args) => args.run(&cli).await?,
        Commands::Demo(args) => args.run(&cli).await?,
        Commands::Completions(args) => args.run()?,
        Commands::Cache(args) => args.run()?,
    };

    println!("{output}");
    Ok(())
}
