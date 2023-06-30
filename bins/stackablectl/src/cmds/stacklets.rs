use clap::{Args, Subcommand};
use comfy_table::{
    presets::{NOTHING, UTF8_FULL},
    ContentArrangement, Table,
};
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

            // The main table displays all installed (and discovered) stacklets
            // and their condition.
            let mut table = Table::new();
            table
                .set_header(vec!["PRODUCT", "NAME", "NAMESPACE", "CONDITIONS"])
                .set_content_arrangement(ContentArrangement::Dynamic)
                .load_preset(UTF8_FULL);

            // The error table displays optional errors in a structured manner.
            let mut error_table = Table::new();
            error_table
                .set_header(vec!["#", "MESSAGES"])
                .set_content_arrangement(ContentArrangement::Dynamic)
                .load_preset(NOTHING);

            let mut product_index = 0;
            for (product_name, products) in stacklets {
                for product in products {
                    let (conditions, errors) =
                        process_conditions(product.conditions, &mut product_index, use_color);

                    table.add_row(vec![
                        product_name.clone(),
                        product.name,
                        product.namespace.unwrap_or_default(),
                        conditions,
                    ]);

                    for error in errors {
                        error_table.add_row(vec![product_index.to_string(), error]);
                    }
                }
            }

            // Only output the error table if there are errors to report.
            // Currently this is a little awkward, but an upstream PR will make
            // this more straight forward.
            Ok(format!(
                "{}{}",
                table,
                if error_table.row_iter().cloned().count() > 0 {
                    format!("\n\n{}", error_table)
                } else {
                    "".into()
                }
            ))
        }
        OutputType::Json => serde_json::to_string(&stacklets).context(JsonOutputFormatSnafu {}),
        OutputType::Yaml => serde_yaml::to_string(&stacklets).context(YamlOutputFormatSnafu {}),
    }
}

/// Processes conditions for a single stacklet / product. It returns a
/// concatenated string of conditions (which are colored green and red) to
/// display next to each listed stacklet in the table. Additionally, it also
/// returns a list of errors to be displayed underneath the stacklet table.
fn process_conditions(
    product_conditions: Vec<DisplayCondition>,
    product_index: &mut usize,
    use_color: bool,
) -> (String, Vec<String>) {
    let mut conditions = Vec::new();
    let mut errors = Vec::new();

    let mut condition_index = 0;
    for cond in product_conditions {
        match process_condition_error(cond.message, cond.is_good, &mut condition_index, use_color) {
            Some(error) => {
                errors.push(error);
                *product_index += 1;
            }
            None => (),
        }

        conditions.push(color_condition(
            &cond.condition,
            cond.is_good,
            *product_index,
            condition_index,
            use_color,
        ))
    }

    (conditions.join(", "), errors)
}

/// Processes one condition and determines if it is an error (not good). If this
/// is the case, it get colored red and is returned.
fn process_condition_error(
    message: Option<String>,
    is_good: Option<bool>,
    condition_index: &mut usize,
    use_color: bool,
) -> Option<String> {
    let message = message.unwrap_or("-".into());

    match (is_good, use_color) {
        (Some(false), true) => {
            *condition_index += 1;

            Some(
                Red.paint(format!("[{}]: {}", condition_index, message))
                    .to_string(),
            )
        }
        _ => None,
    }
}

/// Colors a single condition (green or red) and additionally adds an error
/// index to the output.
fn color_condition(
    condition: &String,
    is_good: Option<bool>,
    product_index: usize,
    condition_index: usize,
    use_color: bool,
) -> String {
    match (is_good, use_color) {
        (Some(true), true) => Green.paint(condition).to_string(),
        (Some(false), true) => Red
            .paint(format!(
                "{}: See [{}.{}]",
                condition, product_index, condition_index
            ))
            .to_string(),
        _ => condition.to_owned(),
    }
}
