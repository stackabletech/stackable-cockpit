pub mod demos;
pub mod releases;
pub mod root;
pub mod stacklets;
pub mod stacks;
pub mod utoipa_synthetic;

#[cfg(feature = "ui")]
pub mod ui;
#[cfg(not(feature = "ui"))]
#[path = "ui_disabled.rs"]
pub mod ui;
