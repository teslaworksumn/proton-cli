//! This module initializes a project.

use std::io::Write;
use std::path::Path;

use git2::{Oid, Repository, Signature};

use utils;
use Error;
use Project;

/// Initializes a new project at root. The root must either not exist, or must
/// be an empty directory. This will
///
/// 1. Create the directory if it doesn't exist.
/// 2. Create a Protonfile
/// 3. Initialize a git repository and commit the protonfile.
///
/// Impure.
pub fn initialize_project<P: AsRef<Path>>(path: P) -> Result<(), Error> {
    let root = path.as_ref();
    let signature = Signature::now("Proton Lights", "proton@teslaworks.net").unwrap();

    utils::create_empty_directory(root)
        .and_then(|_| make_protonfile(root))
        .and_then(|_| make_repository(root))
        .and_then(|repo| initial_commit(&repo, &signature))
        .map(|_| ())
}

/// Writes an empty Protonfile to the root.
///
/// Impure.
fn make_protonfile(root: &Path) -> Result<(), Error> {
    let project = Project::empty();
    utils::write_protonfile(&project, Some(root))
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
