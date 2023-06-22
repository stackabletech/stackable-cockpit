use clap::{Args, Subcommand};
use comfy_table::{presets::UTF8_FULL, ContentArrangement, Table};
use nu_ansi_term::Color::{Green, Red};
use snafu::{ResultExt, Snafu};
use tracing::{info, instrument};

use stackable::{
    kube::DisplayCondition,
    platform::stacklet::{list_stacklets, StackletError},
};

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

    if stacklets.is_empty() {
        return Ok("No stacklets".into());
    }

    match args.output_type {
        OutputType::Plain => {
            // Determine if colored output will be enabled based on the provided
            // flag and the terminal support.
            let use_color = use_colored_output(args.use_color);

            let mut all_messages = Vec::new();
            let mut table = Table::new();

            table
                .set_header(vec!["PRODUCT", "NAME", "NAMESPACE", "CONDITIONS"])
                .set_content_arrangement(ContentArrangement::Dynamic)
                .load_preset(UTF8_FULL);

            let mut index = 0;
            for (product_name, products) in stacklets {
                for product in products {
                    let (conditions, messages) =
                        format_product_conditions(product.conditions, use_color, index);

                    table.add_row(vec![
                        product_name.clone(),
                        product.name,
                        product.namespace.unwrap_or_default(),
                        conditions,
                    ]);

                    all_messages.extend(messages);
                    index += 1;
                }
            }

            Ok(format!(
                "{}{}",
                table,
                if all_messages.is_empty() {
                    "".into()
                } else {
                    format!("\n\n{}", all_messages.join("\n"))
                }
            ))
        }
        OutputType::Json => serde_json::to_string(&stacklets).context(JsonOutputFormatSnafu {}),
        OutputType::Yaml => serde_yaml::to_string(&stacklets).context(YamlOutputFormatSnafu {}),
    }
}

/// This formats the product conditions for display in a table.
fn format_product_conditions(
    conditions: Vec<DisplayCondition>,
    use_color: bool,
    index: usize,
) -> (String, Vec<String>) {
    let mut messages = Vec::new();

    let formatted = conditions
        .iter()
        .map(|cond| match (cond.is_good, use_color) {
            (Some(is_good), true) => {
                if is_good {
                    Green.paint(&cond.condition).to_string()
                } else {
                    messages.push(
                        Red.paint(format!(
                            "[{}]: {}",
                            index,
                            cond.message.clone().unwrap_or("No message".into())
                        ))
                        .to_string(),
                    );
                    Red.paint(format!("{}: See [{}]", &cond.condition, index))
                        .to_string()
                }
            }
            (None, _) | (Some(_), _) => cond.condition.to_owned(),
        })
        .collect::<Vec<String>>()
        .join(", ");

    (formatted, messages)
}
