use clap::Args;
use stackable_cockpit::platform::operator::listener_operator::ListenerClassPreset;

#[derive(Debug, Args)]
#[command(next_help_heading = "Operator specific configurations")]
pub struct CommonOperatorConfigsArgs {
    /// Choose the ListenerClass preset (`none`, `ephemeral-nodes` or `stable-nodes`).
    ///
    /// This maps to the listener-operator preset, see
    /// [the listener-operator documentation](https://docs.stackable.tech/home/nightly/listener-operator/listenerclass/#presets)
    /// for details.
    ///
    /// This argument is likely temporary until we support setting arbitrary helm values for the
    /// operators!
    #[arg(long, global = true)]
    pub listener_class_preset: Option<ListenerClassPreset>,
}
