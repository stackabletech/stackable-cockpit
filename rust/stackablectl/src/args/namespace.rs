use clap::Args;
use stackable_cockpit::constants::{DEFAULT_OPERATOR_NAMESPACE, DEFAULT_PRODUCT_NAMESPACE};

#[derive(Debug, Args)]
#[command(next_help_heading = "Namespace options")]
pub struct CommonNamespaceArgs {
    /// Namespace where the operators are deployed
    #[arg(long, global = true, visible_aliases(["operator-ns"]), long_help = format!(
        "Namespace where the operators are deployed\n\n\
        If this option is not provided, stackablectl will use the default value\n\
        '{}'.", DEFAULT_OPERATOR_NAMESPACE
    ))]
    pub operator_namespace: Option<String>,

    /// Namespace where the products (e.g. stacks or demos) are deployed
    #[arg(short = 'n', long, global = true, visible_aliases(["product-ns"]), long_help = format!(
        "Namespace where the products (e.g. stacks or demos) are deployed\n\n\
        If this option is not provided, stackablectl will use the default value\n\
        '{}'.", DEFAULT_PRODUCT_NAMESPACE
    ))]
    pub product_namespace: Option<String>,
}
