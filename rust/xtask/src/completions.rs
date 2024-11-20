use std::fs;

use clap::CommandFactory;
use clap_complete::{
    generate as generate_comps,
    Shell::{Bash, Elvish, Fish, Zsh},
};
use clap_complete_nushell::Nushell;
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
    generate_comps(Bash, &mut cmd, name.clone(), &mut f);

    // Elvish completions
    let mut f = fs::File::create("extra/completions/stackablectl.elv").context(IoSnafu)?;
    generate_comps(Elvish, &mut cmd, name.clone(), &mut f);

    // Fish completions
    let mut f = fs::File::create("extra/completions/stackablectl.fish").context(IoSnafu)?;
    generate_comps(Fish, &mut cmd, name.clone(), &mut f);

    // Nushell completions
    let mut f = fs::File::create("extra/completions/stackablectl.nu").context(IoSnafu)?;
    generate_comps(Nushell, &mut cmd, name.clone(), &mut f);

    // ZSH completions
    let mut f = fs::File::create("extra/completions/_stackablectl").context(IoSnafu)?;
    generate_comps(Zsh, &mut cmd, name, &mut f);

    Ok(())
}
