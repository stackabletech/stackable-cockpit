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

            // The main table displays all installed (and discovered) stacklets
            // and their condition.
            let mut table = Table::new();
            table
                .set_header(vec!["PRODUCT", "NAME", "NAMESPACE", "CONDITIONS"])
                .set_content_arrangement(ContentArrangement::Dynamic)
                .load_preset(UTF8_FULL);

            // The error table displays optional errors in a structured manner.
            let mut error_index = 1;
            let mut error_list = Vec::new();

            for (product_name, products) in stacklets {
                for product in products {
                    let (conditions, errors) =
                        render_conditions(product.conditions, &mut error_index, use_color);

                    table.add_row(vec![
                        product_name.clone(),
                        product.name,
                        product.namespace.unwrap_or_default(),
                        conditions,
                    ]);

                    match render_errors(errors) {
                        Some(err) => error_list.push(err),
                        None => (),
                    }
                }
            }

            // Only output the error table if there are errors to report.
            Ok(format!(
                "{table}{errors}",
                errors = if error_list.len() > 0 {
                    format!("\n\n{}", error_list.join("\n"))
                } else {
                    "".into()
                }
            ))
        }
        OutputType::Json => serde_json::to_string(&stacklets).context(JsonOutputFormatSnafu {}),
        OutputType::Yaml => serde_yaml::to_string(&stacklets).context(YamlOutputFormatSnafu {}),
    }
}

/// Renders conditions for a single stacklet / product. It returns a
/// concatenated string of conditions (which are colored green and red) to
/// display next to each listed stacklet in the table. Additionally, it also
/// returns a list of errors to be displayed underneath the stacklet table.
fn render_conditions(
    product_conditions: Vec<DisplayCondition>,
    error_index: &mut usize,
    use_color: bool,
) -> (String, Vec<String>) {
    let mut conditions = Vec::new();
    let mut errors = Vec::new();

    for cond in product_conditions {
        conditions.push(color_condition(
            &cond.condition,
            cond.is_good,
            *error_index,
            use_color,
        ));

        match render_condition_error(cond.message, cond.is_good, *error_index, use_color) {
            Some(error) => {
                errors.push(error);
                *error_index += 1;
            }
            None => (),
        };
    }

    (conditions.join(", "), errors)
}

/// Renders one condition and determines if it is an error (not good). If this
/// is the case, it get colored red and is returned.
fn render_condition_error(
    message: Option<String>,
    is_good: Option<bool>,
    error_index: usize,
    use_color: bool,
) -> Option<String> {
    let message = message.unwrap_or("-".into());

    match (is_good, use_color) {
        (Some(false), true) => Some(
            Red.paint(format!("[{}]: {}", error_index, message))
                .to_string(),
        ),
        _ => None,
    }
}

/// Colors a single condition (green or red) and additionally adds an error
/// index to the output.
fn color_condition(
    condition: &str,
    is_good: Option<bool>,
    error_index: usize,
    use_color: bool,
) -> String {
    match (is_good, use_color) {
        (Some(true), true) => Green.paint(condition).to_string(),
        (Some(false), true) => Red
            .paint(format!("{}: See [{}]", condition, error_index))
            .to_string(),
        _ => condition.to_owned(),
    }
}

/// Renders multiple errors (of one stacklet)
fn render_errors(errors: Vec<String>) -> Option<String> {
    if errors.len() == 0 {
        None
    } else if errors.len() == 1 {
        Some(errors[0].clone())
    } else {
        Some(format!("{}\n---\n", errors.join("\n")))
    }
}
