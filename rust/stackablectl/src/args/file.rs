use clap::{Args, ValueHint};

#[derive(Debug, Args)]
#[command(next_help_heading = "File options")]
pub struct CommonFileArgs {
    /// Provide one or more additional (custom) demo file(s)
    #[arg(short, long = "demo-file", value_name = "DEMO_FILE", value_hint = ValueHint::FilePath, global = true)]
    #[arg(long_help = "Provide one or more additional (custom) demo file(s)

Demos are loaded in the following order: Remote (default) demo file, custom
demo files provided via the 'STACKABLE_DEMO_FILES' environment variable, and
lastly demo files provided via the '-d/--demo-file' argument(s). If there are
demos with the same name, the last demo definition will be used.

Use \"stackablectl [OPTIONS] <COMMAND> -d path/to/demos1.yaml -d path/to/demos2.yaml\"
to provide multiple additional demo files.")]
    pub demo_files: Vec<String>,

    /// Provide one or more additional (custom) stack file(s)
    #[arg(short, long = "stack-file", value_name = "STACK_FILE", value_hint = ValueHint::FilePath, global = true)]
    #[arg(long_help = "Provide one or more additional (custom) stack file(s)

Stacks are loaded in the following order: Remote (default) stack file, custom
stack files provided via the 'STACKABLE_STACK_FILES' environment variable, and
lastly demo files provided via the '-s/--stack-file' argument(s). If there are
stacks with the same name, the last stack definition will be used.

Use \"stackablectl [OPTIONS] <COMMAND> -s path/to/stacks1.yaml -s path/to/stacks2.yaml\"
to provide multiple additional stack files.")]
    pub stack_files: Vec<String>,

    /// Provide one or more additional (custom) release file(s)
    #[arg(short, long = "release-file", value_name = "RELEASE_FILE", value_hint = ValueHint::FilePath, global = true)]
    #[arg(long_help = "Provide one or more additional (custom) release file(s)

Releases are loaded in the following order: Remote (default) release file,
custom release files provided via the 'STACKABLE_RELEASE_FILES' environment
variable, and lastly release files provided via the '-r/--release-file'
argument(s). If there are releases with the same name, the last release
definition will be used.

Use \"stackablectl [OPTIONS] <COMMAND> -r path/to/releases1.yaml -r path/to/releases2.yaml\"
to provide multiple additional release files.")]
    pub release_files: Vec<String>,
}
