// External crates
use clap::{Args, Subcommand};
use comfy_table::{presets::UTF8_FULL, ContentArrangement, Table};
use snafu::{ResultExt, Snafu};

// Stackable library
use stackable::platform::stacklet::{list_stacklets, StackletError, StackletListOptions};
use tracing::{info, instrument};

// Local
use crate::cli::{Cli, OutputType};

#[derive(Debug, Args)]
pub struct StackletsArgs {
    #[command(subcommand)]
    subcommand: StackletCommands,
}

#[derive(Debug, Subcommand)]
pub enum StackletCommands {
    /// List deployed services
    #[command(alias("ls"))]
    List(StackletListArgs),
}

#[derive(Debug, Args)]
pub struct StackletListArgs {
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
pub enum StackletsCmdError {
    #[snafu(display("service list error"))]
    StackletListError { source: StackletError },

    #[snafu(display("unable to format yaml output"))]
    YamlOutputFormatError { source: serde_yaml::Error },

    #[snafu(display("unable to format json output"))]
    JsonOutputFormatError { source: serde_json::Error },
}

impl StackletsArgs {
    pub async fn run(&self, common_args: &Cli) -> Result<String, StackletsCmdError> {
        match &self.subcommand {
            StackletCommands::List(args) => list_cmd(args, common_args).await,
        }
    }
}

#[instrument]
async fn list_cmd(args: &StackletListArgs, common_args: &Cli) -> Result<String, StackletsCmdError> {
    info!("Listing installed stacklets");

    // If the user wants to list stacklets from all namespaces, we use `None`.
    // `None` indicates that don't want to list stacklets scoped to only ONE
    // namespace.
    let namespace = args
        .all_namespaces
        .then_some(common_args.operator_namespace.as_str());

    let stacklets = list_stacklets(namespace, StackletListOptions::default())
        .await
        .context(StackletListSnafu {})?;

    println!("{:?}", stacklets);

    match args.output_type {
        OutputType::Plain => {
            let mut table = Table::new();

            table
                .set_header(vec!["PRODUCT", "NAME", "NAMESPACE", "ENDPOINTS", "INFO"])
                .set_content_arrangement(ContentArrangement::Dynamic)
                .load_preset(UTF8_FULL);

            for (product_name, products) in stacklets {
                for product in products {
                    let endpoints = product
                        .endpoints
                        .iter()
                        .map(|(name, url)| format!("{name}:{url}"))
                        .collect::<Vec<String>>()
                        .join("\n");

                    let additional_information = product
                        .additional_information
                        .iter()
                        .map(|(key, value)| format!("{key}:{value}"))
                        .collect::<Vec<String>>()
                        .join("\n");

                    table.add_row(vec![
                        product_name.clone(),
                        product.name,
                        product.namespace.unwrap_or_default(),
                        endpoints,
                        additional_information,
                    ]);
                }
            }

            Ok(table.to_string())
        }
        OutputType::Json => todo!(),
        OutputType::Yaml => todo!(),
    }
}
