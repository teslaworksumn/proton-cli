
use std::io::Cursor;
use openssl::crypto::rsa::RSA as openssl_RSA;

use error::Error;
use project_types::Permission;

// Note: DO NOT return a uid in any public function. They are used for authentication and are
// for internal function calls only.

#[derive(Clone, Debug, Eq, RustcEncodable, RustcDecodable)]
pub struct User {
    pub uid: u32,
    pub name: String,
    pub public_key: String,
}

impl PartialEq for User {
    fn eq(&self, other: &User) -> bool {
        self.uid == other.uid ||
        self.public_key == other.public_key
    }
}

impl User {

    /// Validates the public key, then creates a new User with that key and a unique id
    pub fn new(name: &str, pub_key: &str) -> Result<User, Error> {
        try!(User::validate_public_key(&pub_key));

        // Make unique user id
        let uid = 0;

        Ok(User {
            name: name.to_string(),
            uid: uid,
            public_key: pub_key.to_string(),
        })
    }

    /// Checks if the given public key is valid
    pub fn validate_public_key(pub_key: &str) -> Result<(), Error> {
        let mut pub_key_readable = Cursor::new(pub_key.to_string());
        openssl_RSA::public_key_from_pem(&mut pub_key_readable)
            .map(|_| ())
            .map_err(|_| Error::InvalidPublicKey(pub_key.to_string()))
    }

}
