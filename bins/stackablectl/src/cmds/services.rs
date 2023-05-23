// External crates
use clap::{Args, Subcommand};
use comfy_table::{presets::UTF8_FULL, ContentArrangement, Table};
use snafu::{ResultExt, Snafu};

// Stackable library
use stackable::platform::service::{list_services, ServiceError, ServiceListOptions};
use tracing::{info, instrument};

// Local
use crate::cli::{Cli, OutputType};

#[derive(Debug, Args)]
pub struct ServicesArgs {
    #[command(subcommand)]
    subcommand: ServiceCommands,
}

#[derive(Debug, Subcommand)]
pub enum ServiceCommands {
    /// List deployed services
    #[command(alias("ls"))]
    List(ServiceListArgs),
}

#[derive(Debug, Args)]
pub struct ServiceListArgs {
    /// Will display services of all namespaces, not only the current one
    #[arg(short, long)]
    all_namespaces: bool,

    /// Display credentials and secrets in the output
    #[arg(short, long)]
    show_credentials: bool,

    /// Display product versions in the output
    #[arg(long)]
    show_versions: bool,

    #[arg(short, long = "output", value_enum, default_value_t = Default::default())]
    output_type: OutputType,
}

#[derive(Debug, Snafu)]
pub enum ServicesCmdError {
    #[snafu(display("service list error"))]
    ServiceListError { source: ServiceError },

    #[snafu(display("unable to format yaml output"))]
    YamlOutputFormatError { source: serde_yaml::Error },

    #[snafu(display("unable to format json output"))]
    JsonOutputFormatError { source: serde_json::Error },
}

impl ServicesArgs {
    pub fn run(&self, common_args: &Cli) -> Result<String, ServicesCmdError> {
        match &self.subcommand {
            ServiceCommands::List(args) => list_cmd(args, common_args),
        }
    }
}

#[instrument]
fn list_cmd(args: &ServiceListArgs, common_args: &Cli) -> Result<String, ServicesCmdError> {
    info!("Listing installed services");

    // If the user wants to list services from all namespaces, we use `None`.
    // `None` indicates that don't want to list services scoped to only ONE
    // namespace.
    let namespace = if args.all_namespaces {
        None
    } else {
        Some(common_args.operator_namespace.as_str())
    };

    let services =
        list_services(namespace, ServiceListOptions::default()).context(ServiceListSnafu {})?;

    match args.output_type {
        OutputType::Plain => {
            let mut table = Table::new();

            table
                .set_header(vec!["PRODUCT", "NAME", "NAMESPACE", "ENDPOINTS", "INFO"])
                .set_content_arrangement(ContentArrangement::Dynamic)
                .load_preset(UTF8_FULL);

            for (_product_name, _installed_products) in services {
                // table.add_row(vec![
                //     product_name,
                //     installed_products.name,
                //     installed_products.namespace.unwrap_or_default(),
                // ]);
            }

            Ok(table.to_string())
        }
        OutputType::Json => todo!(),
        OutputType::Yaml => todo!(),
    }
}
