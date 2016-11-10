use std::path::Path;

use project_types::User;
use dao::{UserDao, UserDaoPostgres};
use error::Error;


impl UserDao for UserDaoPostgres {

    fn add_initial_user(&self, private_key: &str) -> Result<(), Error> {
        Err(Error::TodoErr)
    }

    fn get_user(&self, uid: u32) -> Result<User, Error> {
        Err(Error::TodoErr)
    }

    /// Identifies a user by their public SSH key by finding the
    /// corresponding private key in the project. This public key
    /// acts like the user's password, and should be protected.
    /// 
    /// Impure.
    fn get_user_id<P: AsRef<Path>>(&self, public_key_path: P) -> Result<u32, Error> {
        Err(Error::TodoErr)
    }
    
}
