//! This module initializes a project.

use std::fs;
use std::io::{Result, Error, ErrorKind};

use git2::Repository;

/// Creates a folder. The folder must not exist or must be empty.
///
/// Impure.
pub fn make_project_folder(root: &str) -> Result<()> {
    // Make the folder - ignore error.
    let _ = fs::create_dir(root);

    // Check that the folder is empty
    fs::read_dir(root)
        .and_then(|iter| {
            let count = iter.count();
            if count == 0 {
                Ok(())
            } else {
                dir_not_empty_err(root, count)
            }
        })
}

fn dir_not_empty_err(root: &str, count: usize) -> Result<()> {
    Err(Error::new(ErrorKind::Other, format!("{} was not empty: {} files exist", root, count)))
}

/// Initializes a git repository at root.
///
/// Impure.
pub fn make_repository(root: &str) -> Result<()> {
    Ok(())
}