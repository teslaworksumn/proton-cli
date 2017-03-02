use dao::{UserDao, UserDaoPostgres};
use error::Error;
use project_types::User;


impl UserDao for UserDaoPostgres {

    fn add_initial_user(&self, proj_name: &str, private_key: &str, public_key: &str) -> Result<u32, Error> {
        let root_uname = format!("{}_{}", "root", proj_name);
        self.add_user(&root_uname, private_key, public_key)
    }

    fn add_user(&self, name: &str, private_key: &str, public_key: &str) -> Result<u32, Error> {
        let statement = "INSERT INTO users (name, private_key, public_key) VALUES ($1, $2, $3)";
        let private_string = private_key.trim_matches('\n');
        let public_string = public_key.trim_matches('\n');
        let _ = try!(
            self.conn.execute(
                statement,
                &[&name.to_owned(), &private_string, &public_string])
            .map_err(Error::Postgres));
        let uid = try!(self.get_user_id(&public_string));
        Ok(uid)
    }

    /// Identifies a user by their public SSH key
    ///
    /// Impure.
    fn get_user_id(&self, public_key: &str) -> Result<u32, Error> {
        let public_string = public_key.trim_matches('\n');
        let query = "SELECT uid FROM users WHERE public_key = $1";
        let results = try!(
            self.conn.query(query, &[&public_string])
            .map_err(Error::Postgres));
        
        match results.len() {
            0 => Err(Error::PublicKeyNotFound(public_key.to_owned())),
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
