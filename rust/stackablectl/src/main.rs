use clap::Parser;
use dotenvy::dotenv;
use stackable_cockpit::PROGRESS_SPINNER_STYLE;
use stackablectl::cli::{Cli, Error};
use tracing::{Level, metadata::LevelFilter};
use tracing_indicatif::{
    IndicatifLayer,
    filter::{IndicatifFilter, hide_indicatif_span_fields},
    indicatif_eprintln,
};
use tracing_subscriber::{
    Layer as _,
    fmt::{self, format::DefaultFields},
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

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
        .with_span_field_formatter(
            // If the `{span_fields}` interpolation is used in a template, then we want to hide the
            // indicatif control fields "indicatif.pb_show" and "indicatif.pb_hide"
            hide_indicatif_span_fields(DefaultFields::new()),
        )
        .with_progress_style(PROGRESS_SPINNER_STYLE.clone());

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
            .with(indicatif_layer.with_filter(IndicatifFilter::new(false)))
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

            eprint!("{}", output.render());
            std::process::exit(1);
        }
    }

    Ok(())
}
