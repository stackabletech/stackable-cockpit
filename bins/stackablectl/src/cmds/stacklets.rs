use clap::{Args, Subcommand};
use comfy_table::{presets::UTF8_FULL, ContentArrangement, Table};
use nu_ansi_term::Color::{Green, Red};
use snafu::{ResultExt, Snafu};
use tracing::{info, instrument};

use stackable::platform::stacklet::{list_stacklets, StackletError};

use crate::{
    cli::{Cli, OutputType},
    utils::use_colored_output,
};

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

    /// Controls if the output will use color. This only applies to the output
    /// type 'plain'.
    #[arg(short = 'c', long = "color")]
    use_color: bool,

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

    let stacklets = list_stacklets(namespace)
        .await
        .context(StackletListSnafu {})?;

    match args.output_type {
        OutputType::Plain => {
            // Determine if colored output will be enabled based on the provided
            // flag and the terminal support.
            let use_color = use_colored_output(args.use_color);

            let mut table = Table::new();

            table
                .set_header(vec!["PRODUCT", "NAME", "NAMESPACE", "CONDITIONS"])
                .set_content_arrangement(ContentArrangement::Dynamic)
                .load_preset(UTF8_FULL);

            for (product_name, products) in stacklets {
                for product in products {
                    let conditions = format_product_conditions(product.conditions, use_color);

                    table.add_row(vec![
                        product_name.clone(),
                        product.name,
                        product.namespace.unwrap_or_default(),
                        conditions,
                    ]);
                }
            }

            Ok(table.to_string())
        }
        OutputType::Json => serde_json::to_string(&stacklets).context(JsonOutputFormatSnafu {}),
        OutputType::Yaml => serde_yaml::to_string(&stacklets).context(YamlOutputFormatSnafu {}),
    }
}

/// This formats the product conditions for display in a table.
fn format_product_conditions(conditions: Vec<(String, Option<bool>)>, use_color: bool) -> String {
    conditions
        .iter()
        .map(|(cond, is_good)| match (is_good, use_color) {
            (Some(is_good), true) => {
                if *is_good {
                    Green.paint(cond).to_string()
                } else {
                    Red.paint(cond).to_string()
                }
            }
            (None, _) | (Some(_), _) => cond.to_owned(),
        })
        .collect::<Vec<String>>()
        .join(", ")
}
