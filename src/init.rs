//! This module initializes a project.

use std::fs;
use git2::Repository;
use Error;

/// Creates a folder. The folder must not exist or must be empty.
///
/// Impure.
pub fn make_project_folder(root: &str) -> Result<(), Error> {
    // Make the folder - ignore error.
    let _ = fs::create_dir(root);

    // Check that the folder is empty
    fs::read_dir(root)
        .map_err(|err| Error::Io(err))
        .and_then(|iter| {
            let count = iter.count();
            if count == 0 {
                Ok(())
            } else {
                Err(Error::FolderNotEmpty(root, count))
            }
        })
}

/// Initializes a git repository at root.
///
/// Impure.
pub fn make_repository(root: &str) -> Result<(), Error> {
    Repository::init(root)
        .map_err(|err| Error::Git(err))
        .map(|_| ())
}