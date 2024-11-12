use clap::Parser;
use dotenvy::dotenv;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::fmt;

use stackablectl::cli::{Cli, Commands, Error};

#[snafu::report]
#[tokio::main]
async fn main() -> Result<(), Error> {
    // Parse the CLI args and commands
    let app = Cli::parse();

    // Catch if --offline is used for now
    if app.offline {
        todo!()
    }

    // The control center does it's own logging, we don't want to mess up the screen for it
    if !matches!(app.subcommand, Commands::ExperimentalControlCenter { .. }) {
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
    }

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
