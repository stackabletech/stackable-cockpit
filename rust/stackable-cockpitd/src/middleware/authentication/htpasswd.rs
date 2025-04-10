use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use snafu::{ResultExt, Snafu};

use super::{PasswordHash, Username};

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("failed to read htpasswd file at {path:?}"))]
    Read {
        source: std::io::Error,
        path: PathBuf,
    },
    #[snafu(display("malformed htpasswd entry on line {line}"))]
    Entry { source: EntryError, line: usize },
}
#[derive(Debug, Snafu)]
pub enum EntryError {
    #[snafu(display("invalid hash type (only bcrypt is currently supported)"))]
    InvalidHashType,
    #[snafu(display("no username/password separator"))]
    NoSeparator,
}

pub(super) fn load(path: &Path) -> Result<HashMap<Username, PasswordHash>, Error> {
    let htpasswd = std::fs::read_to_string(path).context(ReadSnafu { path })?;
    parse(&htpasswd)
}

pub(super) fn parse(htpasswd: &str) -> Result<HashMap<Username, PasswordHash>, Error> {
    let mut accounts = HashMap::new();
    for (line, entry) in htpasswd.lines().enumerate() {
        let (username, hash) = parse_entry(entry).context(EntrySnafu { line: line + 1 })?;
        accounts.insert(username, hash);
    }
    Ok(accounts)
}

fn parse_entry(entry: &str) -> Result<(Username, PasswordHash), EntryError> {
    if let Some((username, prefixed_pw_hash)) = entry.split_once(':') {
        if prefixed_pw_hash.starts_with("$2y$") {
            Ok((
                Username(username.to_string()),
                PasswordHash::Bcrypt(prefixed_pw_hash.to_string()),
            ))
        } else {
            InvalidHashTypeSnafu.fail()
        }
    } else {
        NoSeparatorSnafu.fail()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{EntryError, Error, parse};
    use crate::middleware::authentication::{PasswordHash, Username};

    #[test]
    fn test_load_htaccess() {
        assert_eq!(
            parse(
                "foo:$2y$05$egqCB7MQNkL5BSpFnhwtjO4huCzTaxQliSQkghug7DoqEfVjjFFy.
bar:$2y$05$EUss9LlzDDCWmGVOKzKYwue2KR2BubDXjUhu3Ih8iBC.9MDgkpT26"
            )
            .unwrap(),
            HashMap::from([
                (
                    Username("foo".to_string()),
                    PasswordHash::Bcrypt(
                        "$2y$05$egqCB7MQNkL5BSpFnhwtjO4huCzTaxQliSQkghug7DoqEfVjjFFy.".to_string()
                    )
                ),
                (
                    Username("bar".to_string()),
                    PasswordHash::Bcrypt(
                        "$2y$05$EUss9LlzDDCWmGVOKzKYwue2KR2BubDXjUhu3Ih8iBC.9MDgkpT26".to_string()
                    )
                )
            ])
        );

        let htpasswd_with_invalid_hash =
            "foo:$2y$05$egqCB7MQNkL5BSpFnhwtjO4huCzTaxQliSQkghug7DoqEfVjjFFy.
bar:$8y$05$EUss9LlzDDCWmGVOKzKYwue2KR2BubDXjUhu3Ih8iBC.9MDgkpT26";
        assert!(matches!(
            parse(htpasswd_with_invalid_hash),
            Err(Error::Entry {
                line: 2,
                source: EntryError::InvalidHashType
            })
        ));

        let htpasswd_with_missing_separator =
            "foo:$2y$05$egqCB7MQNkL5BSpFnhwtjO4huCzTaxQliSQkghug7DoqEfVjjFFy.
bar$2y$05$EUss9LlzDDCWmGVOKzKYwue2KR2BubDXjUhu3Ih8iBC.9MDgkpT26";
        assert!(matches!(
            parse(htpasswd_with_missing_separator),
            Err(Error::Entry {
                line: 2,
                source: EntryError::NoSeparator
            })
        ));
    }
}
