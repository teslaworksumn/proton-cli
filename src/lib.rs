extern crate git2;
extern crate rustc_serialize;

use std::path::Path;
use git2::Signature;

mod init;
mod project_types;
mod error;

// Re-export all error code into the this namespace
pub use error::*;

/// Initializes a new project at root. The root must either not exist, or must
/// be an empty directory. This will
///
/// 1. Create the directory if it doesn't exist.
/// 2. Create a Protonfile
/// 3. Initialize a git repository and commit the protonfile.
///
/// Impure.
pub fn initialize_project(root: &Path, signature: &Signature) -> Result<(), Error> {
    init::make_project_folder(root)
        .and_then(|_| init::make_protonfile(root))
        .and_then(|_| init::make_repository(root))
        .and_then(|repo| init::initial_commit(&repo, &signature))
        .map(|_| ())
}
