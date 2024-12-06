use clap::Parser;
use dotenvy::dotenv;
use tracing::metadata::LevelFilter;
use tracing_subscriber::fmt;

use stackablectl::cli::{Cli, Error};

#[snafu::report]
#[tokio::main]
async fn main() -> Result<(), Error> {
    // Parse the CLI args and commands
    let app = Cli::parse();

    // Construct the tracing subscriber
    let format = fmt::format()
        .with_ansi(true)
        .without_time()
        .with_target(false);

    tracing_subscriber::fmt()
        .with_max_level(match app.log_level {
            Some(level) => LevelFilter::from_level(level),
            None => LevelFilter::WARN,
        })
        .event_format(format)
        .pretty()
        .init();

    // Load env vars from optional .env file
    match dotenv() {
        Ok(_) => (),
        Err(err) => {
            if !err.not_found() {
                println!("{err}")
            }
        }
    }

    match app.run().await {
        Ok(result) => print!("{result}"),
        Err(err) => {
            let mut output = app.error();
            output.with_error_report(err);

            eprint!("{}", output.render())
        }
    }

    Ok(())
}
