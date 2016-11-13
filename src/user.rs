//! This module manages project users
use std::path::Path;
use std::fs::File;
use std::io::Cursor;

use git2::Signature;

use dao::{PermissionDao, UserDao};
use error::Error;
use project_types::User;
use utils;

/// Creates a new user for the project in the current directory.
/// Generates a public/private key pair for the new user
/// and returns the public key.
///
/// Impure.
pub fn new_user<P: AsRef<Path>, UD: UserDao, PD: PermissionDao>(
    user_dao: UD,
    perm_dao: PD,
    admin_key_path: P,
    name: &str
) -> Result<String, Error> {

    let (root_pub_key, root_private_key) = try!(utils::create_pub_priv_keys());

    // See if admin has permission to add user
    let admin_uid = try!(user_dao.get_user_id(admin_key_path.as_ref()));
    let admin = try!(user_dao.get_user(admin_uid));
    let admin_permissions = try!(perm_dao.get_all_permissions(admin_uid))

    Err(Error::TodoErr)

    // Add user
    // Commit changes

    // let signature = Signature::now("Proton Lights", "proton@teslaworks.net").unwrap();
    // let msg = format!("Adding {} as new user", name);
    // let pf_path = Path::new("Protonfile.json");
    // let repo_path: Option<P> = None;

    // utils::commit_file(&pf_path, repo_path, &signature, &msg)
}

/// Removes a user from the project in the current directory
/// Assumes the current directory contains a Protonfile.json file.
///
/// Impure.
pub fn remove_user<P: AsRef<Path>>(
    admin_key_path: P,
    uid: u32
) -> Result<(), Error> {

    Err(Error::TodoErr)
    // See if admin has permission to add user
    // Can't remove root
    // Remove user

    // Commit changes

    // let signature = Signature::now("Proton Lights", "proton@teslaworks.net").unwrap();
    // let msg = format!("Removing user {}", name);
    // let pf_path = Path::new("Protonfile.json");
    // let repo_path: Option<P> = None;

    // utils::commit_file(&pf_path, repo_path, &signature, &msg)
}

