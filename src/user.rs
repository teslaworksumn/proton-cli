//! This module manages project users
use std::path::Path;
use std::fs::File;
use std::io::Cursor;
use openssl::crypto::rsa::RSA as openssl_RSA;
use openssl::crypto::hash::Type as openssl_HashType;

use git2::Signature;

use error::Error;
use project_types::User;
use utils;

/// Creates a new user for the project in the current directory.
/// Assumes the current directory contains a Protonfile.json file.
///
/// 1. Read the new user's public key from the file path given
/// 2. Add the user's name and public key to the protonfile
///
/// Impure.
pub fn new_user<P: AsRef<Path>>(
    admin_key_path: P,
    public_key_path: P,
    name: &str
) -> Result<(), Error> {

    Err(Error::TodoErr)
    // See if admin has permission to add user
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

