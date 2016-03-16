extern crate git2;

pub mod init;
pub mod project_types;

// Re-export all error code into the top namespace
mod error;
pub use error::*;
