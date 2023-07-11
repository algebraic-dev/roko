//! This module renders a virtual dom to a the real dom, does patching and reconciliation.

pub mod app;
pub mod diff;
pub mod patch;
pub mod render;

pub use web_sys as dom;
