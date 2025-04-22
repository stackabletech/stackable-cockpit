use clap::Args;
use stackable_cockpit::constants::{DEFAULT_NAMESPACE, DEFAULT_OPERATOR_NAMESPACE};

#[derive(Debug, Args)]
#[command(next_help_heading = "Namespace options")]
pub struct CommonNamespaceArgs {
    /// Namespace where the operators are deployed
    #[arg(long, global = true, visible_aliases(["operator-ns"]), default_value = DEFAULT_OPERATOR_NAMESPACE, long_help = "Namespace where the operators are deployed")]
    pub operator_namespace: String,

    /// Namespace where the stacks or demos are deployed
    #[arg(short = 'n', long, global = true, visible_aliases(["product-ns"]), aliases(["product-namespace"]), default_value = DEFAULT_NAMESPACE, long_help = "Namespace where the stacks or demos are deployed")]
    pub namespace: String,
}
