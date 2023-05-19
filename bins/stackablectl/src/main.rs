use clap::Parser;
use dotenvy::dotenv;
use tracing::metadata::LevelFilter;
use tracing_subscriber::fmt;

use stackablectl::cli::{self, Commands};

#[tokio::main]
async fn main() {
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

    match &cli.subcommand {
        Commands::Operator(args) => match args.run(&cli).await {
            Ok(out) => println!("{out}"),
            Err(err) => eprintln!("{err}"),
        },
        Commands::Release(args) => match args.run() {
            Ok(out) => println!("{out}"),
            Err(err) => eprintln!("{err}"),
        },
        Commands::Stack(args) => match args.run(&cli).await {
            Ok(out) => println!("{out}"),
            Err(err) => eprintln!("{err}"),
        },
        Commands::Services(args) => match args.run() {
            Ok(out) => println!("{out}"),
            Err(err) => eprintln!("{err}"),
        },
        Commands::Demo(args) => match args.run(&cli).await {
            Ok(out) => println!("{out}"),
            Err(err) => eprintln!("{err}"),
        },
        Commands::Completions(args) => match args.run() {
            Ok(out) => println!("{out}"),
            Err(err) => eprintln!("{err}"),
        },
        Commands::Cache(args) => match args.run() {
            Ok(out) => println!("{out}"),
            Err(err) => eprintln!("{err}"),
        },
    };
}
