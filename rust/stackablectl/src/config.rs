use std::path::{Path, PathBuf};

use serde::Deserialize;
use snafu::{ResultExt, Snafu};

#[derive(Debug, Default, Deserialize)]
pub struct UserConfig {
    pub version: VersionOptions,
}

#[derive(Debug, Deserialize)]
pub struct VersionOptions {
    pub check_enabled: bool,
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("failed to read config file from {path}", path = path.display()))]
    Read {
        source: std::io::Error,
        path: PathBuf,
    },

    #[snafu(display("failed to deserialize config file located at {path} as TOML", path = path.display()))]
    Deserialize {
        source: toml::de::Error,
        path: PathBuf,
    },
}

impl UserConfig {
    pub fn from_file<P>(path: P) -> Result<Option<Self>, Error>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();

        match std::fs::read_to_string(path) {
            Ok(contents) => {
                let config = toml::from_str(&contents).context(DeserializeSnafu { path })?;
                Ok(Some(config))
            }
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(Error::Read {
                path: path.to_path_buf(),
                source: err,
            }),
        }
    }
}

impl Default for VersionOptions {
    fn default() -> Self {
        Self {
            check_enabled: true,
        }
    }
}
