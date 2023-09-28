use std::fs;

use clap::CommandFactory;
use clap_complete::{generate as generate_comps, Shell};
use snafu::{ResultExt, Snafu};
use stackablectl::cli::Cli;

#[derive(Debug, Snafu)]
pub enum GenCompError {
    #[snafu(display("io error"))]
    Io { source: std::io::Error },
}

pub fn generate() -> Result<(), GenCompError> {
    let mut cmd = Cli::command();
    let name = cmd.get_name().to_string();

    fs::create_dir_all("extra/completions").context(IoSnafu)?;

    // Bash completions
    let mut f = fs::File::create("extra/completions/stackablectl.bash").context(IoSnafu)?;
    generate_comps(Shell::Bash, &mut cmd, name.clone(), &mut f);

    // Fish completions
    let mut f = fs::File::create("extra/completions/stackablectl.fish").context(IoSnafu)?;
    generate_comps(Shell::Fish, &mut cmd, name.clone(), &mut f);

    // ZSH completions
    let mut f = fs::File::create("extra/completions/_stackablectl").context(IoSnafu)?;
    generate_comps(Shell::Zsh, &mut cmd, name, &mut f);

    Ok(())
}
