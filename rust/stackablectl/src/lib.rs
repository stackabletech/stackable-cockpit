pub mod args;
pub mod cli;
pub mod cmds;
pub mod constants;
pub mod output;
pub mod release_check;
pub mod utils;

pub mod built_info {
    use std::{str::FromStr, sync::LazyLock};

    include!(concat!(env!("OUT_DIR"), "/built.rs"));

    pub static PKG_SEMVER: LazyLock<semver::Version> = LazyLock::new(|| {
        semver::Version::from_str(PKG_VERSION).expect("must be a valid semantic version")
    });
}
