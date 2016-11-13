use postgres::types::ToSql;
use std::path::Path;

use dao::{UserDao, UserDaoPostgres};
use error::Error;
use project_types::User;
use utils;


impl UserDao for UserDaoPostgres {

    fn add_initial_user(&self, private_key: &str, public_key: &str) -> Result<(), Error> {
        let statement = "INSERT INTO users (name, private_key, public_key) VALUES ($1, $2, $3)";
        let private_string = private_key.trim_matches('\n');
        let public_string = public_key.trim_matches('\n');
        let rows_modified = try!(
            self.conn.execute(
                statement,
                &[&"root".to_owned(), &private_string, &public_string])
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
        let public_key = try!(utils::file_as_string(public_key_path.as_ref()));
        let public_string = public_key.trim_matches('\n');
        let query = "SELECT uid FROM users WHERE public_key = $1";
        let results = try!(
            self.conn.query(query, &[&public_string])
            .map_err(Error::Postgres));
        
        match results.len() {
            0 => Err(Error::AdminNotFound),
            1 => {
                let row = results.get(0);
                println!("row: {:?}", row);
                let uid: i32 = row.get(0);
                Ok(uid as u32)
            },
            x => Err(Error::InvalidNumResults(x)),
        }
    }
    
}
