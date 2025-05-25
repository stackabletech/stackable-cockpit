use std::sync::LazyLock;

use indicatif::ProgressStyle;

pub mod common;
pub mod constants;
pub mod engine;
pub mod helm;
pub mod oci;
pub mod platform;
pub mod utils;
pub mod xfer;

pub static PROGRESS_BAR_STYLE: LazyLock<ProgressStyle> = LazyLock::new(|| {
    ProgressStyle::with_template("{span_child_prefix}Progress: {wide_bar} {pos}/{len}")
        .expect("valid progress template")
});

pub static PROGRESS_SPINNER_STYLE: LazyLock<ProgressStyle> = LazyLock::new(|| {
    ProgressStyle::with_template("{span_child_prefix}{spinner} {msg}")
        .expect("valid progress template")
});
