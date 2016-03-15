//! This module initializes a project.

use std::{fs, io, error};

use git2::Repository;

#[derive(Debug)]
pub enum InitError<'a> {
    Io(io::Error),
    FolderNotEmpty(&'a str, usize),
    TodoErr
}

impl error::Error for InitError {
    fn description(&self) -> &str {
        match self {
            Io(err) => format!("IoError occurred: {}", err.description),
            FolderNotEmpty(root, count) => format!("{} was not empty: {} files exist", root, count),
            TodoErr => "TodoErr",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self {
            Io(err) => Some(err),
            FolderNotEmpty(_, _) => None,
            TodoErr => None,
        }
    }
}

/// Creates a folder. The folder must not exist or must be empty.
///
/// Impure.
pub fn make_project_folder(root: &str) -> Result<(), InitError> {
    // Make the folder - ignore error.
    let _ = fs::create_dir(root);

    // Check that the folder is empty
    fs::read_dir(root)
        .map_err(|err| InitError::Io(err))
        .and_then(|iter| {
            let count = iter.count();
            if count == 0 {
                Ok(())
            } else {
                Err(InitError::FolderNotEmpty(root, count))
            }
        })
}

/// Initializes a git repository at root.
///
/// Impure.
pub fn make_repository(root: &str) -> Result<Repository, InitError> {
    unimplemented!()
}