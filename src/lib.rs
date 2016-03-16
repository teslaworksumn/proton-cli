extern crate git2;

pub mod init;
pub mod error;

// Re-export all error code into the top namespace
pub use error::*;
