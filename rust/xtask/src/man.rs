use std::fs;

use clap::CommandFactory;
use clap_mangen::Man;
use snafu::{ResultExt, Snafu};
use stackablectl::cli::Cli;

#[derive(Debug, Snafu)]
pub enum GenManError {
    #[snafu(display("io error"))]
    Io { source: std::io::Error },
}

pub fn generate() -> Result<(), GenManError> {
    let cmd = Cli::command();

    fs::create_dir_all("extra/man").context(IoSnafu)?;
    let mut f = fs::File::create("extra/man/stackablectl.1").context(IoSnafu)?;

    let man = Man::new(cmd);
    man.render(&mut f).context(IoSnafu)
}
