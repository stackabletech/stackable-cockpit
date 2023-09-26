use clap::{Args, Subcommand};
use comfy_table::{
    presets::{NOTHING, UTF8_FULL},
    ContentArrangement, Table,
};
use nu_ansi_term::Color::{Green, Red};
use snafu::{ResultExt, Snafu};
use tracing::{info, instrument};

use stackable_cockpit::{
    constants::DEFAULT_PRODUCT_NAMESPACE,
    platform::stacklet::{get_credentials_for_product, list_stacklets, StackletError},
    utils::k8s::DisplayCondition,
};

use crate::{
    args::CommonNamespaceArgs,
    cli::{Cli, OutputType},
    utils::use_colored_output,
};

const CREDENTIALS_HINT: &str = "Use \"stackablectl stacklet credentials [OPTIONS] <PRODUCT_NAME> <STACKLET_NAME>\" to display credentials for deployed stacklets.";

#[derive(Debug, Args)]
pub struct StackletArgs {
    #[command(subcommand)]
    subcommand: StackletCommands,
}

#[derive(Debug, Subcommand)]
pub enum StackletCommands {
    /// Display credentials for a stacklet
    #[command(aliases(["creds", "cr"]))]
    Credentials(StackletCredentialsArgs),

    /// List deployed stacklets
    #[command(alias("ls"))]
    List(StackletListArgs),
}

#[derive(Debug, Args)]
pub struct StackletCredentialsArgs {
    /// The name of the product, for example 'superset'.
    product_name: String,

    /// The name of the stacklet, for example 'superset'.
    stacklet_name: String,

    /// Namespace in the cluster used to deploy the products.
    #[arg(
        long,
        short = 'n',
        global = true,
        default_value = DEFAULT_PRODUCT_NAMESPACE,
        visible_aliases(["product-ns"]),
        long_help = "Namespace in the cluster used to deploy the products. Use this to select
a different namespace for credential lookup.")]
    pub product_namespace: String,
}

#[derive(Debug, Args)]
pub struct StackletListArgs {
    /// Controls if the output will use color. This only applies to the output
    /// type 'plain'.
    #[arg(short = 'c', long = "color")]
    use_color: bool,

    /// Display credentials for various endpoints. This requires permissions to
    /// read Kubernetes secrets. These credentials provide access to deployed
    /// stacklets and thus should be handled with care.
    #[arg(long)]
    show_credentials: bool,

    #[arg(short, long = "output", value_enum, default_value_t)]
    output_type: OutputType,

    #[command(flatten)]
    namespaces: CommonNamespaceArgs,
}

#[derive(Debug, Snafu)]
pub enum CmdError {
    #[snafu(display("failed to list stacklets"))]
    StackletListError { source: StackletError },

    #[snafu(display("failed to retrieve credentials for stacklet"))]
    StackletCredentialsError { source: StackletError },

    #[snafu(display("unable to format yaml output"))]
    YamlOutputFormatError { source: serde_yaml::Error },

    #[snafu(display("unable to format json output"))]
    JsonOutputFormatError { source: serde_json::Error },
}

impl StackletArgs {
    pub async fn run(&self, common_args: &Cli) -> Result<String, CmdError> {
        match &self.subcommand {
            StackletCommands::List(args) => list_cmd(args, common_args).await,
            StackletCommands::Credentials(args) => credentials_cmd(args).await,
        }
    }
}

#[instrument]
async fn list_cmd(args: &StackletListArgs, common_args: &Cli) -> Result<String, CmdError> {
    info!("Listing installed stacklets");

    // If the user wants to list stacklets from all namespaces, we use `None`.
    // `None` indicates that don't want to list stacklets scoped to only ONE
    // namespace.
    let stacklets = list_stacklets(args.namespaces.product_namespace.as_deref())
        .await
        .context(StackletListSnafu)?;

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
                .set_header(vec![
                    "PRODUCT",
                    "NAME",
                    "NAMESPACE",
                    "ENDPOINTS",
                    "CONDITIONS",
                ])
                .set_content_arrangement(ContentArrangement::Dynamic)
                .load_preset(UTF8_FULL);

            let mut error_list = Vec::new();
            let mut error_index = 1;

            let max_endpoint_name_length = stacklets
                .iter()
                .flat_map(|s| &s.endpoints)
                .map(|(endpoint_name, _)| endpoint_name.len())
                .max()
                .unwrap_or_default();

            for stacklet in stacklets {
                let ConditionOutput { summary, errors } =
                    render_conditions(stacklet.conditions, &mut error_index, use_color);

                let endpoints = stacklet
                    .endpoints
                    .iter()
                    .map(|(name, url)| {
                        format!("{name:width$}{url}", width = max_endpoint_name_length + 1)
                    })
                    .collect::<Vec<_>>()
                    .join("\n");

                table.add_row(vec![
                    stacklet.product,
                    stacklet.name,
                    stacklet.namespace.unwrap_or_default(),
                    endpoints,
                    summary,
                ]);

                if let Some(err) = render_errors(errors) {
                    error_list.push(err)
                }
            }

            // Only output the error list if there are errors to report.
            Ok(format!(
                "{table}{errors}\n\n{CREDENTIALS_HINT}",
                errors = if !error_list.is_empty() {
                    format!("\n\n{}", error_list.join("\n"))
                } else {
                    "".into()
                }
            ))
        }
        OutputType::Json => serde_json::to_string(&stacklets).context(JsonOutputFormatSnafu),
        OutputType::Yaml => serde_yaml::to_string(&stacklets).context(YamlOutputFormatSnafu),
    }
}

#[instrument]
async fn credentials_cmd(args: &StackletCredentialsArgs) -> Result<String, CmdError> {
    info!("Displaying stacklet credentials");

    match get_credentials_for_product(
        &args.product_namespace,
        &args.stacklet_name,
        &args.product_name,
    )
    .await
    .context(StackletCredentialsSnafu)?
    {
        Some(credentials) => {
            let mut table = Table::new();

            table
                .set_content_arrangement(ContentArrangement::Dynamic)
                .load_preset(NOTHING)
                .add_row(vec!["USERNAME", &credentials.username])
                .add_row(vec!["PASSWORD", &credentials.password]);

            let output = format!(
                "Credentials for {} ({}) in namespace '{}':",
                args.product_name, args.stacklet_name, args.product_namespace
            );

            Ok(format!("{}\n\n{}", output, table))
        }
        None => Ok("No credentials".into()),
    }
}

pub struct ConditionOutput {
    summary: String,
    errors: Vec<String>,
}

/// Renders conditions for a single stacklet / product. It returns a
/// concatenated string of conditions (which are colored green and red) to
/// display next to each listed stacklet in the table. Additionally, it also
/// returns a list of errors to be displayed underneath the stacklet table.
fn render_conditions(
    product_conditions: Vec<DisplayCondition>,
    error_index: &mut usize,
    use_color: bool,
) -> ConditionOutput {
    let mut conditions = Vec::new();
    let mut errors = Vec::new();

    for cond in product_conditions {
        conditions.push(color_condition(
            &cond.condition,
            cond.is_good,
            *error_index,
            use_color,
        ));

        if let Some(error) =
            render_condition_error(cond.message, cond.is_good, *error_index, use_color)
        {
            errors.push(error);
            *error_index += 1;
        };
    }

    ConditionOutput {
        summary: conditions.join(", "),
        errors,
    }
}

/// Renders one condition and determines if it is an error (not good). If this
/// is the case, it get colored red and is returned.
fn render_condition_error(
    message: Option<String>,
    is_good: Option<bool>,
    error_index: usize,
    use_color: bool,
) -> Option<String> {
    if !is_good.unwrap_or(true) {
        let message = message.unwrap_or("-".into());
        let mut error = format!("[{error_index}]: {message}");

        if use_color {
            error = Red.paint(error).to_string()
        }

        return Some(error);
    }

    None
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
            .paint(format!("{condition}: See [{error_index}]"))
            .to_string(),
        (Some(false), false) => format!("{condition}: See [{error_index}]"),
        _ => condition.to_owned(),
    }
}

/// Renders multiple errors (of one stacklet)
fn render_errors(errors: Vec<String>) -> Option<String> {
    if errors.is_empty() {
        None
    } else if errors.len() == 1 {
        Some(errors[0].clone())
    } else {
        Some(format!("{}\n---\n", errors.join("\n")))
    }
}
