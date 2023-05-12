use tracing::{debug, instrument};
use which::which;

use std::ffi::OsStr;

/// Returns if the binary with `name` is present in the $PATH.
pub fn binary_present<T: AsRef<OsStr>>(name: T) -> bool {
    which(name).is_ok()
}

/// Returns if ALL binaries in the list are present in the $PATH.
#[instrument]
pub fn binaries_present<T, L>(names: L) -> bool
where
    T: AsRef<OsStr> + std::fmt::Debug,
    L: AsRef<[T]> + std::fmt::Debug,
{
    debug!("Checking if required binaries are present on the system");

    for name in names.as_ref() {
        if !binary_present(name) {
            return false;
        }
    }

    true
}
