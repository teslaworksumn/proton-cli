//! This module manages project users
use std::path::Path;

use dao::{PermissionDao, UserDao};
use error::Error;
use project_types::PermissionEnum;
use utils;


pub fn get_user_id<P: AsRef<Path>, UD: UserDao>(
    user_dao: UD,
    public_key_path: P
) -> Result<u32, Error> {
    
    let public_key_str = try!(utils::file_as_string(public_key_path.as_ref()));

    // Throw error if the given key is not a valid RSA public key
    if !utils::validate_rsa_pub_key(&public_key_str) {
        return Err(Error::InvalidPublicKey(public_key_str));
    }

    // Lookup uid
    let uid = try!(user_dao.get_user_id(&public_key_str));

    Ok(uid)
}

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

    // See if admin has permission to add user
    let valid_permissions = vec![PermissionEnum::Administrate];
    let _ = try!(utils::check_valid_permission(
        &perm_dao,
        &user_dao,
        admin_key_path,
        &valid_permissions));

    // Create keys
    let (user_pub_key, user_private_key) = try!(utils::create_pub_priv_keys());

    // Add user
    let _ = try!(user_dao.add_user(name, &user_private_key, &user_pub_key));

    // Return public key
    Ok(user_pub_key)
}

/// Removes a user from the project in the current directory
/// Assumes the current directory contains a Protonfile.json file.
///
/// Impure.
#[allow(unused_variables)]
pub fn remove_user<P: AsRef<Path>>(
    admin_key_path: P,
    uid: u32
) -> Result<(), Error> {

    Err(Error::TodoErr)
    // See if admin has permission to add user
    // Can't remove root
    // Remove user

}

