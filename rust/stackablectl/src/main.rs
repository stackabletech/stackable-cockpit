use clap::Parser;
use dotenvy::dotenv;
use indicatif::ProgressStyle;
use stackablectl::cli::{Cli, Error};
use tracing::metadata::LevelFilter;
use tracing_indicatif::{indicatif_println, IndicatifLayer};
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
        .with_progress_style(ProgressStyle::with_template("").unwrap())
        .with_max_progress_bars(
            15,
            Some(
                ProgressStyle::with_template(
                    "...and {pending_progress_bars} more processes not shown above.",
                )
                .unwrap(),
            ),
        );

    let level_filter = match app.log_level {
        Some(level) => LevelFilter::from_level(level),
        None => LevelFilter::INFO,
    };

    if level_filter == LevelFilter::DEBUG {
        tracing_subscriber::registry()
            .with(
                fmt::layer()
                    .event_format(format)
                    .pretty()
                    .with_writer(indicatif_layer.get_stderr_writer()),
            )
            .init();
    } else {
        tracing_subscriber::registry()
            .with(level_filter)
            .with(indicatif_layer)
            .init();
    }

    // Load env vars from optional .env file
    match dotenv() {
        Ok(_) => (),
        Err(err) => {
            if !err.not_found() {
                indicatif_println!("{err}")
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
