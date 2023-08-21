use std::{fs, path::Path};

use clap::CommandFactory;
use snafu::{ResultExt, Snafu};
use stackablectl::cli::Cli;

const USAGE_STRING: &str = "Command line tool to interact with the Stackable Data Platform\n\nUsage: stackablectl [OPTIONS] <COMMAND>\n";

#[derive(Debug, Snafu)]
pub enum GenReadmeError {
    #[snafu(display("io error"))]
    Io { source: std::io::Error },
}

pub fn generate() -> Result<(), GenReadmeError> {
    let mut cmd = Cli::command();
    let usage_text = cmd.render_long_help().to_string();
    let usage_text: Vec<_> = usage_text.lines().map(|l| l.trim_end()).collect();
    let usage_text = usage_text.join("\n");

    let readme_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("stackablectl/README.md");

    let mut readme = fs::read_to_string(&readme_path).context(IoSnafu)?;
    let usage_start = readme.find(USAGE_STRING).unwrap();
    let usage_end = readme[usage_start..].find("\n```").unwrap();

    readme.replace_range(usage_start..usage_start + usage_end, &usage_text);
    fs::write(readme_path, readme).context(IoSnafu)
}
