//! This module initializes a project.

use std::{fs, path};
use std::io::Write;

use rustc_serialize::json;
use git2::Repository;

use Error;
use project_types::Project;

/// Creates a folder. The folder must not exist or must be empty.
///
/// Impure.
pub fn make_project_folder(root: &str) -> Result<(), Error> {
    // Make the folder - ignore error.
    let _ = fs::create_dir(root);

    // Check that the folder is empty
    fs::read_dir(root)
        .map_err(Error::Io)
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
pub fn make_repository(root: &str) -> Result<Repository, Error> {
    Repository::init(root)
        .map_err(Error::Git)
}

/// 
///
/// Impure.
pub fn make_protonfile(root: &str) -> Result<(), Error> {
    let project = Project::empty();
    let pretty_json = json::as_pretty_json(&project);

    let mut path = path::PathBuf::from(root);
    path.push("Protonfile.json");

    let mut protonfile = try!(fs::File::create(path));
    write!(&mut protonfile, "{}\n", pretty_json)
        .map_err(Error::Io)
}
