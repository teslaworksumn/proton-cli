//! This module manages project users
use std::path::Path;

use dao::{UserDao};
use error::Error;
use utils;


/// Lookup and return a user's user id from a public key
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

pub fn new_user<UD: UserDao>(
    user_dao: UD,
    name: &str
) -> Result<String, Error> {

    // Create keys
    let (user_pub_key, user_private_key) = try!(utils::create_pub_priv_keys());

    // Add user
    let _ = try!(user_dao.add_user(name, &user_private_key, &user_pub_key));

    // Return public key
    Ok(user_pub_key)
}

/// Removes a user
#[allow(unused_variables)]
pub fn remove_user(
    uid: u32
) -> Result<(), Error> {

    Err(Error::TodoErr)
    // See if admin has permission to add user
    // Can't remove root
    // Remove user

}