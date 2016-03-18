//! This module initializes a project.

use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use rustc_serialize::json;
use git2::{Oid, Repository, Signature};

use Error;
use project_types::Project;

/// Initializes a new project at root. The root must either not exist, or must
/// be an empty directory. This will
///
/// 1. Create the directory if it doesn't exist.
/// 2. Create a Protonfile
/// 3. Initialize a git repository and commit the protonfile.
///
/// Impure.
pub fn initialize_project<P: AsRef<Path>>(path: P, signature: &Signature) -> Result<(), Error> {
    let root = path.as_ref();
    make_project_folder(root)
        .and_then(|_| make_protonfile(root))
        .and_then(|_| make_repository(root))
        .and_then(|repo| initial_commit(&repo, &signature))
        .map(|_| ())
}

/// Creates a folder. The folder must not exist or must be empty.
///
/// Impure.
fn make_project_folder(root: &Path) -> Result<(), Error> {
    // Make the folder - ignore error.
    let _ = fs::create_dir(root);

    // Check that the folder is empty
    fs::read_dir(root)
        .map(|iter| iter.count())
        .map_err(Error::Io)
        .and_then(|count|
            if count == 0 {
                Ok(())
            } else {
                Err(folder_not_empty(root, count))
            })
}

fn folder_not_empty(root: &Path, count: usize) -> Error {
    Error::FolderNotEmpty(root.to_str().unwrap().to_owned(), count)
}

/// Writes an empty Protonfile to the root.
///
/// Impure.
fn make_protonfile(root: &Path) -> Result<(), Error> {
    // Create content
    let project = Project::empty();
    let pretty_json = json::as_pretty_json(&project);

    // Make path
    let mut path = PathBuf::from(root);
    path.push("Protonfile.json");

    File::create(&path)
        .and_then(|mut protonfile| write!(protonfile, "{}\n", pretty_json))
        .map_err(Error::Io)
}

/// Initializes a git repository at root.
///
/// Impure.
fn make_repository(root: &Path) -> Result<Repository, Error> {
    Repository::init(root)
        .map_err(Error::Git)
}

/// Stages the Protonfile and makes an initial commit.
///
/// Impure.
fn initial_commit(repo: &Repository, signature: &Signature) -> Result<Oid, Error> {
    let path = Path::new("Protonfile.json");

    repo.index()
        .and_then(|mut index| index.add_path(&path).map(|_| index))
        .and_then(|mut index| index.write().map(|_| index))
        .and_then(|mut index| index.write_tree())
        .and_then(|oid| repo.find_tree(oid))
        .and_then(|tree| repo.commit(
            Some("HEAD"),
            signature,
            signature,
            "Initial commit.",
            &tree,
            &vec![]))
        .map_err(Error::Git)
}
