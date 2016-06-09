//! This module manages project users
extern crate openssl;

use std::path::Path;
use std::fs::File;
use std::io::Cursor;
use self::openssl::crypto::rsa::RSA as openssl_RSA;
use self::openssl::crypto::hash::Type as openssl_HashType;
use self::openssl::ssl::error::SslError as openssl_Error;

use git2::Signature;

use Error;
use User;
use utils;


/// Creates a new user for the project in the current directory.
/// Assumes the current directory contains a Protonfile.json file.
///
/// 1. Read the new user's public key from the file path given
/// 2. Add the user's name and public key to the protonfile
///
/// Impure.
pub fn new_user<P: AsRef<Path>>(public_key_path: P, name: &str) -> Result<(), Error> {
    // Add user
    let pub_key = try!(get_public_key(public_key_path));
    let project = try!(utils::read_protonfile(None::<P>));
    let new_project = try!(project.add_user(&name, &pub_key));
    try!(utils::write_protonfile(&new_project, None::<P>));

    // Commit changes
    let signature = Signature::now("Proton Lights", "proton@teslaworks.net").unwrap();
    let msg = format!("Adding {} as new user", name);
    let pf_path = Path::new("Protonfile.json");
    let repo_path: Option<P> = None;

    utils::commit_file(&pf_path, repo_path, &signature, &msg)
}
/// Identifies a user by their private SSH key by finding the
/// corresponding public key in the project. This private key
/// acts like the user's password, and should be protected.
/// 
/// Impure.
pub fn id_user<P: AsRef<Path>>(private_key_path: P) -> Result<User, Error> {
    let test_data: &[u8] = b"Testing to find private/public key pair";
    
    let mut private_key_file = try!(File::open(&private_key_path).map_err(Error::Io));
    
    let private_key = try!(openssl_RSA::private_key_from_pem(&mut private_key_file)
        .map_err(ssl_error));

    let signature = try!(private_key.sign(openssl_HashType::MD5, &test_data)
        .map_err(ssl_error));

    let project = try!(utils::read_protonfile(None::<P>));
    for user in project.users {
        let user_key = user.public_key.clone();
        let mut pub_key_readable = Cursor::new(&user_key);

        let rsa_public = try!(openssl_RSA::public_key_from_pem(&mut pub_key_readable)
            .map_err(Error::Ssl));
        
        match rsa_public.verify(openssl_HashType::MD5, &test_data, &signature) {
            Ok(valid) => if valid {
                return Ok(user)
            },
            Err(e) => return Err(ssl_error(e)),
        };
    };
    
    Err(Error::UserNotFound)
}

fn get_public_key<P: AsRef<Path>>(public_key_path: P) -> Result<String, Error> {
    let pub_key = try!(utils::file_as_string(public_key_path));
    let mut pub_key_readable = Cursor::new(&pub_key);
    match openssl_RSA::public_key_from_pem(&mut pub_key_readable) {
        Ok(_) => Ok(pub_key.clone()),
        Err(_) => Err(invalid_pub_key_error(&pub_key)),
    }
}

fn invalid_pub_key_error(key: &str) -> Error {
    Error::InvalidPublicKey(key.to_string())
}

fn ssl_error(e: openssl_Error) -> Error {
    Error::Ssl(e)
}

