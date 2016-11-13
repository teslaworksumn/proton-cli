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

    /// Identifies a user by their public SSH key by finding the
    /// public key in the database. This public key
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
                let uid: i32 = row.get(0);
                Ok(uid as u32)
            },
            x => Err(Error::InvalidNumResults(x)),
        }
    }
    
    fn get_user(&self, uid: u32) -> Result<User, Error> {
        let query = "SELECT name, public_key FROM users WHERE uid = $1";
        let uid_i32 = uid as i32;
        let results = try!(
            self.conn.query(query, &[&uid_i32])
            .map_err(Error::Postgres));
        match results.len() {
            0 => Err(Error::UserNotFound),
            1 => {
                let row = results.get(0);
                let name: String = row.get(0);
                let public_key: String = row.get(1);
                Ok(User {
                    uid: uid,
                    name: name,
                    public_key: public_key
                })
            },
            x => Err(Error::InvalidNumResults(x)),
        }
    }

}
