use clap::Args;
use stackable_cockpit::platform::operator::listener_operator::ListenerOperatorPreset;

#[derive(Debug, Args)]
#[command(next_help_heading = "Operator specific configurations")]
pub struct CommonOperatorConfigsArgs {
    /// Choose the listener-operator preset (`none`, `ephemeral-nodes` or `stable-nodes`).
    ///
    /// See [the listener-operator documentation](https://docs.stackable.tech/home/nightly/listener-operator/listenerclass/#presets)
    /// for details.
    ///
    /// This argument is temporary until we support setting arbitrary helm values for the operators!
    #[arg(long, global = true)]
    pub listener_operator_preset: Option<ListenerOperatorPreset>,
}
