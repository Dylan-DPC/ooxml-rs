//! Documents implementations.
//!
//! A document is a OPC package with specific components.
mod presentation;
pub mod spreadsheet;
mod wordprocessing;

pub use spreadsheet::*;
