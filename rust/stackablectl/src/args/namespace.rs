use clap::Args;
use stackable_cockpit::constants::{DEFAULT_OPERATOR_NAMESPACE, DEFAULT_PRODUCT_NAMESPACE};

#[derive(Debug, Args)]
#[command(next_help_heading = "Namespace options")]
pub struct CommonNamespaceArgs {
    /// Namespace in the cluster used to deploy the operators
    #[arg(short = 'n', long, default_value = DEFAULT_OPERATOR_NAMESPACE, global = true)]
    pub operator_namespace: String,

    /// Namespace in the cluster used to deploy the products
    #[arg(long, default_value = DEFAULT_PRODUCT_NAMESPACE, global = true)]
    pub product_namespace: String,
}
