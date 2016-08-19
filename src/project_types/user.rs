
use std::io::Cursor;
use openssl::crypto::rsa::RSA as openssl_RSA;

use error::Error;
use project_types::Permission;


#[derive(Clone, Debug, Eq, RustcEncodable, RustcDecodable)]
pub struct User {
    pub name: String,
    pub public_key: String,
    pub permissions: Vec<Permission>,
}

impl PartialEq for User {
    fn eq(&self, other: &User) -> bool {
        self.name == other.name ||
        self.public_key == other.public_key
    }
}

impl User {

    /// Validates the public key, then creates a new User with that key
    pub fn new(name: &str, pub_key: &str) -> Result<User, Error> {
        try!(User::validate_public_key(&pub_key));

        Ok(User {
            name: name.to_string(),
            public_key: pub_key.to_string(),
            permissions: Vec::new(),
        })
    }

    /// Checks if the given public key is valid
    pub fn validate_public_key(pub_key: &str) -> Result<(), Error> {
        let mut pub_key_readable = Cursor::new(pub_key.to_string());
        openssl_RSA::public_key_from_pem(&mut pub_key_readable)
            .map(|_| ())
            .map_err(|_| Error::InvalidPublicKey(pub_key.to_string()))
    }

    /// Adds the given permission to the user's list of permissions
    /// If it already exists, this becomes a NOP
    pub fn add_permission(&mut self, perm: Permission) {
        if !self.has_permission(&perm) {
            self.permissions.push(perm);
        }
    }

    /// Removes the given permission from the User's list of permissions
    /// If it isn't found, this becomes a NOP
    pub fn remove_permission(&mut self, perm: Permission) {
        for i in 0..self.permissions.len() {
            if self.permissions[i] == perm {
                self.permissions.remove(i);
                break;
            }
        }
    }

    /// Checks to see if the user has the given Permission
    pub fn has_permission(&self, perm: &Permission) -> bool {
        
        for p in &self.permissions {
            if p == perm {
                return true;
            }
        }

        false
    }

    /// Determines whether the user has the Administrate permission
    pub fn is_admin(&self) -> bool {
        for p in &self.permissions {
            if p == &Permission::Administrate {
                return true;
            }
        }

        false
    }

}
