use std::fs::{self, File};
use std::io::Read;
use std::path::Path;

use openssl::rsa;
use openssl::pkey;
use rustc_serialize::json;

use dao::{PermissionDao, UserDao};
use project_types::PermissionEnum;
use error::Error;

/// Converts a JSON 2d sequence array into a Vec<Vec<u16>>
pub fn sequence_json_to_vec(seq: json::Json) -> Vec<Vec<u16>> {
    seq
        .as_array()
        .unwrap()
        .iter()
        .map(|row| {
            row
                .as_array()
                .unwrap()
                .iter()
                .map(|v| {
                    v.as_i64().unwrap() as u16
                }).collect::<Vec<u16>>()
        }).collect::<Vec<Vec<u16>>>()
}

/// Creates a new public/private key pair
pub fn create_pub_priv_keys() -> Result<(String, String), Error> {
    let keys = try!(rsa::Rsa::generate(2048).map_err(Error::Ssl));
    let pkey = try!(pkey::PKey::from_rsa(keys).map_err(Error::Ssl));
    let private_key = try!(pkey.private_key_to_pem().map_err(Error::Ssl));
    let public_key = try!(pkey.public_key_to_pem().map_err(Error::Ssl));
    let private_key_str = String::from_utf8(private_key).expect("Generated private key not UTF-8");
    let public_key_str = String::from_utf8(public_key).expect("Generated public key not UTF-8");
    Ok((public_key_str, private_key_str))
}

/// Checks that the given string is a valid RSA public key
pub fn validate_rsa_pub_key(pub_key: &str) -> bool {
    rsa::Rsa::public_key_from_pem(&pub_key.bytes().collect::<Vec<u8>>())
        .is_ok()
}

/// Checks if the user with a public key at the given path has
/// one of the given valid permissions
/// Returns this user if found and has permission, else error
pub fn check_valid_permission<P: AsRef<Path>, PD: PermissionDao, UD: UserDao>(
    perm_dao: &PD,
    user_dao: &UD,
    public_key_path: P,
    valid_permissions: &Vec<PermissionEnum>
) -> Result<u32, Error> {
    
    if valid_permissions.len() > 0 {
        let public_key = try!(file_as_string(public_key_path));
        let uid = try!(user_dao.get_user_id(&public_key));
        let permissions = try!(perm_dao.get_all_permissions(uid));
        for permission in permissions {
            if valid_permissions.contains(&permission.permission) {
                return Ok(uid);
            }
        }
    }

    Err(Error::UnauthorizedAction)
}

/// Returns the last part of the path, the file name, if no problems arise
/// Raises errors if the file name is invalid or cannot be converted to UTF-8
pub fn file_name_from_path<P: AsRef<Path>>(path: P) -> Result<String, Error> {
    match path.as_ref().file_name() {
        Some(name_os) => {
            match name_os.to_str() {
                Some(name) => Ok(name.to_string()),
                None => Err(Error::InvalidFileName),
            }
        },
        None => Err(Error::InvalidFileName),
    }
}

/// Creates a folder. The folder must not exist or must be empty.
///
/// Impure.
pub fn create_empty_directory<P: AsRef<Path>>(dir_path: P) -> Result<(), Error> {
    // Make the folder - ignore error.
    let _ = fs::create_dir(&dir_path);

    // Check that the folder is empty
    fs::read_dir(&dir_path)
        .map(|iter| iter.count())
        .map_err(Error::Io)
        .and_then(|count|
            if count == 0 {
                Ok(())
            } else {
                Err(Error::FolderNotEmpty(dir_path.as_ref().to_str().unwrap().to_owned(), count))
            })
}

/// Reads a file as a string.
/// Wraps Read::read_to_string errors in proton_cli::Error
pub fn file_as_string<P: AsRef<Path>>(path: P) -> Result<String, Error> {
    if !path.as_ref().exists() {
        return Err(Error::FileNotFound(path.as_ref().to_str().expect("Path not valid UTF-8").to_string()));
    }
    
    File::open(path)
        .and_then(|mut file| {
            let mut string = String::new();
            file.read_to_string(&mut string)
                .and_then(|_| Ok(string.trim().to_string()))           
        })
        .map_err(Error::Io)
}
