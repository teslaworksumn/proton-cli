//! This module initializes a project.

use std::io::Write;
use std::path::Path;

use git2::{Oid, Repository, Signature};

use utils;
use error::Error;
use project_types::Project;
use dao::{LayoutDao, PermissionDao, UserDao};


/// Initializes a new project at root. The root must either not exist, or must
/// be an empty directory. This will
///
/// 1. Create the directory if it doesn't exist.
/// 2. Create a Protonfile
/// 3. Initialize a git repository and commit the protonfile.
///
/// Impure.
pub fn initialize_project<P: AsRef<Path>, UD: UserDao, LD: LayoutDao, PD: PermissionDao>(
    user_dao: UD,
    layout_dao: LD,
    perm_dao: PD,
    root_path: P,
    name: &str
) -> Result<String, Error> {

    let root = root_path.as_ref();
    let (root_pub_key, root_private_key) = try!(utils::create_pub_priv_keys());
    let signature = Signature::now("Proton Lights", "proton@teslaworks.net").unwrap();

    utils::create_empty_directory(root)
        .and_then(|_| make_protonfile(layout_dao, root, name))
        .and_then(|_| make_repository(root))
        .and_then(|repo| initial_commit(&repo, &signature))
        .and_then(|_| user_dao.add_initial_user(&root_private_key, &root_pub_key))
        .and_then(|root_uid| perm_dao.add_initial_permission(root_uid))
        .and_then(|_| Ok(root_pub_key))
}

/// Writes a new Protonfile to the root.
/// It only contains one user, the admin
///
/// Impure.
fn make_protonfile<LD: LayoutDao>(
    layout_dao: LD,
    root: &Path,
    name: &str
) -> Result<(), Error> {

    let project = try!(Project::empty(layout_dao, name, None));
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
