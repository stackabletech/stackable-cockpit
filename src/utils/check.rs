use tracing::{debug, instrument};
use which::which;

use std::ffi::OsStr;

/// Returns if the binary with `name` is present in the $PATH.
pub fn binary_present<T: AsRef<OsStr>>(name: T) -> bool {
    which(name).is_ok()
}

/// Returns if ALL binaries in the list are present in the $PATH.
#[instrument]
pub fn binaries_present(names: &[&str]) -> bool {
    debug!("Checking if required binaries are present on the system");

    for name in names {
        if !binary_present(name) {
            return false;
        }
    }

    true
}

/// Returns [`None`] if all binaries in the list are present in the $PATH and
/// if not, returns [`Some`] containing the name of the missing binary.
#[instrument]
pub fn binaries_present_with_name(names: &[&str]) -> Option<String> {
    debug!("Checking if required binaries are present on the system");

    for name in names {
        if !binary_present(name) {
            return Some(name.to_string());
        }
    }

    None
}
