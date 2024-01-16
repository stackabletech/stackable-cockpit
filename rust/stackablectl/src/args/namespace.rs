use clap::Args;
use stackable_cockpit::constants::{DEFAULT_OPERATOR_NAMESPACE, DEFAULT_PRODUCT_NAMESPACE};

#[derive(Debug, Args)]
#[command(next_help_heading = "Namespace options")]
pub struct CommonNamespaceArgs {
    /// Namespace where the operators are deployed
    #[arg(long, global = true, visible_aliases(["operator-ns"]), default_value = DEFAULT_OPERATOR_NAMESPACE, long_help = "Namespace where the operators are deployed")]
    pub operator_namespace: String,

    /// Namespace where the products (e.g. stacks or demos) are deployed
    #[arg(short = 'n', long, global = true, visible_aliases(["product-ns"]), default_value = DEFAULT_PRODUCT_NAMESPACE, long_help = "Namespace where the products (e.g. stacks or demos) are deployed")]
    pub product_namespace: String,
}
