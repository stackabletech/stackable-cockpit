use clap::Parser;
use dotenvy::dotenv;
use indicatif::ProgressStyle;
use stackablectl::cli::{Cli, Error};
use tracing::{metadata::LevelFilter, Level};
use tracing_indicatif::{indicatif_eprintln, IndicatifLayer};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

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

    let indicatif_layer = IndicatifLayer::new()
        .with_progress_style(ProgressStyle::with_template("").expect("valid progress template"))
        .with_max_progress_bars(
            15,
            Some(
                ProgressStyle::with_template(
                    "...and {pending_progress_bars} more processes not shown above."
                )
                .expect("valid progress template")
            ),
        );

    if let Some(level) = app.log_level {
        tracing_subscriber::registry()
            .with(
                fmt::layer()
                    .event_format(format)
                    .pretty()
                    .with_writer(indicatif_layer.get_stderr_writer()),
            )
            .with(LevelFilter::from_level(level))
            .init();
    } else {
        tracing_subscriber::registry()
            .with(LevelFilter::from_level(Level::INFO))
            .with(indicatif_layer)
            .init();
    }

    // Load env vars from optional .env file
    match dotenv() {
        Ok(_) => (),
        Err(err) => {
            if !err.not_found() {
                indicatif_eprintln!("{err}")
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
