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
pub fn make_project_folder(root: &path::Path) -> Result<(), Error> {
    // Make the folder - ignore error.
    let _ = fs::create_dir(root);

    // Check that the folder is empty
    let count = try!(fs::read_dir(root)).count();
    if count == 0 {
        Ok(())
    } else {
        Err(Error::FolderNotEmpty(root.to_str().unwrap().to_owned(), count))
    }
}

/// Writes an empty Protonfile to the root.
///
/// Impure.
pub fn make_protonfile(root: &path::Path) -> Result<(), Error> {
    let project = Project::empty();
    let pretty_json = json::as_pretty_json(&project);

    let mut path = path::PathBuf::from(root);
    path.push("Protonfile.json");

    let mut protonfile = try!(fs::File::create(path));
    Ok(try!(write!(&mut protonfile, "{}\n", pretty_json)))
}

/// Initializes a git repository at root.
///
/// Impure.
pub fn make_repository(root: &path::Path) -> Result<Repository, Error> {
    Ok(try!(Repository::init(root)))
}

/// Stages the Protonfile and makes an initial commit.
///
/// Impure.
pub fn initial_commit(repo: &Repository) -> Result<(), Error> {
    let index = try!(repo.index());

    Err(Error::TodoErr)
}
