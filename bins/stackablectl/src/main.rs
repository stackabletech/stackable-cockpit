use clap::Parser;
use dotenvy::dotenv;
use snafu::{ResultExt, Snafu};
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

#[derive(Debug, Snafu)]
enum CliError {
    #[snafu(display("operator command error"))]
    OperatorCmdError { source: OperatorCmdError },

    #[snafu(display("release command error"))]
    ReleaseCmdError { source: ReleaseCmdError },

    #[snafu(display("stack command error"))]
    StackCmdError { source: StackCmdError },

    #[snafu(display("services command error"))]
    StackletsCmdError { source: StackletsCmdError },

    #[snafu(display("demo command error"))]
    DemoCmdError { source: DemoCmdError },

    #[snafu(display("completions command error"))]
    CompletionsCmdError { source: CompletionsCmdError },

    #[snafu(display("cache command error"))]
    CacheCmdError { source: CacheCmdError },
}

#[snafu::report]
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
                println!("{err}")
            }
        }
    }

    // Add Helm repos
    if let Err(err) = cli.add_helm_repos() {
        eprintln!("{err}")
    };

    let output = match &cli.subcommand {
        Commands::Operator(args) => args.run(&cli).await.context(OperatorCmdSnafu {})?,
        Commands::Release(args) => args.run(&cli).await.context(ReleaseCmdSnafu {})?,
        Commands::Stack(args) => args.run(&cli).await.context(StackCmdSnafu {})?,
        Commands::Stacklets(args) => args.run(&cli).await.context(StackletsCmdSnafu {})?,
        Commands::Demo(args) => args.run(&cli).await.context(DemoCmdSnafu {})?,
        Commands::Completions(args) => args.run().context(CompletionsCmdSnafu {})?,
        Commands::Cache(args) => args.run().context(CacheCmdSnafu {})?,
    };

    println!("{output}");
    Ok(())
}
