use clap::Parser;
use dotenvy::dotenv;
use indicatif::ProgressStyle;
use stackablectl::cli::{Cli, Error};
use tracing::{Span, metadata::LevelFilter};
use tracing_indicatif::{IndicatifLayer, span_ext::IndicatifSpanExt};
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

    let indicatif_layer = IndicatifLayer::new();

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

        Span::current().pb_set_style(&ProgressStyle::with_template("").unwrap());
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
