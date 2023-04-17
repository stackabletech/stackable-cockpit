use anyhow::Result;
use clap::CommandFactory;
use clap_complete::{generate, Shell};
use clap_mangen::Man;
use stackctl::cli::Cli;

use std::{env, fs};

fn main() -> Result<()> {
    let task = env::args().nth(1);

    match task {
        None => println!("No task specified, available commands: 'gen-man' and 'gen-comp'"),
        Some(t) => match t.as_str() {
            "gen-man" => {
                let cmd = Cli::command();

                fs::create_dir_all("extra/man")?;

                let mut f = fs::File::create("extra/man/stackablectl.1")?;
                let man = Man::new(cmd);
                man.render(&mut f)?;
            }
            "gen-comp" => {
                let mut cmd = Cli::command();
                let name = cmd.get_name().to_string();

                fs::create_dir_all("extra/completions")?;

                // Bash completions
                let mut f = fs::File::create("extra/completions/stackablectl.bash")?;
                generate(Shell::Bash, &mut cmd, name.clone(), &mut f);

                // Fish completions
                let mut f = fs::File::create("extra/completions/stackablectl.fish")?;
                generate(Shell::Fish, &mut cmd, name.clone(), &mut f);

                // ZSH completions
                let mut f = fs::File::create("extra/completions/_stackablectl")?;
                generate(Shell::Zsh, &mut cmd, name, &mut f);
            }
            _ => panic!("Invalid task"),
        },
    }

    Ok(())
}
