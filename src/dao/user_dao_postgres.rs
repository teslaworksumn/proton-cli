use postgres::types::ToSql;
use std::path::Path;

use dao::{UserDao, UserDaoPostgres};
use error::Error;
use project_types::User;


impl UserDao for UserDaoPostgres {

    fn add_initial_user(&self, private_key: &str) -> Result<(), Error> {
        let query = "INSERT INTO users (name, public_key) VALUES ($1, $2)";
        let rows_modified = try!(
            self.conn.execute(query, &[&"root".to_owned(), &private_key.to_owned()])
            .map_err(Error::Postgres));
        Ok(())
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
