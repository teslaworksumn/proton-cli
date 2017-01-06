extern crate proton_cli;

use proton_cli::dao::UserDao;
use proton_cli::error::Error;
use proton_cli::project_types::User;


/// Implementation of UserDao for testing purposes. Uses given functions to return values.
/// Functions are boxed so their sizes are known (pointers).
/// The general naming convention used is trait_function_name_fn, for all trait functions.
/// &str references are converted to Strings so we don't have to deal with lifetime headaches (bookdude13 tried on 12/25/16)
#[allow(dead_code)]
pub struct UserDaoTesting {
	pub add_initial_user_fn: Box<Fn(String, String, String) -> Result<u32, Error>>,
	pub add_user_fn: Box<Fn(String, String, String) -> Result<u32, Error>>,
	pub get_user_fn: Box<Fn(u32) -> Result<User, Error>>,
	pub get_user_id_fn: Box<Fn(String) -> Result<u32, Error>>,
}


impl UserDaoTesting {
	/// Creates a new UserDaoTesting struct with all functions set to return Error::TodoErr
	#[allow(dead_code)]
	pub fn new() -> UserDaoTesting {
		UserDaoTesting {
			add_initial_user_fn: Box::new(|_, _, _| -> Result<u32, Error> { Err(Error::TodoErr) }),
			add_user_fn: Box::new(|_, _, _| -> Result<u32, Error>  { Err(Error::TodoErr) }),
			get_user_fn: Box::new(|_| -> Result<User, Error> { Err(Error::TodoErr) }),
			get_user_id_fn: Box::new(|_| -> Result<u32, Error> { Err(Error::TodoErr) }),
		}
	}
}

/// The Dao implementation simply calls the corresponding stored function
impl UserDao for UserDaoTesting {
	fn add_initial_user(&self, proj_name: &str, private_key: &str, public_key: &str) -> Result<u32, Error> {
		(self.add_initial_user_fn)(proj_name.to_owned(), private_key.to_owned(), public_key.to_owned())
	}

    fn add_user(&self, name: &str, private_key: &str, public_key: &str) -> Result<u32, Error> {
    	(self.add_user_fn)(name.to_owned(), private_key.to_owned(), public_key.to_owned())
    }

    fn get_user(&self, uid: u32) -> Result<User, Error> {
    	(self.get_user_fn)(uid)
    }

    fn get_user_id(&self, public_key: &str) -> Result<u32, Error> {
    	(self.get_user_id_fn)(public_key.to_owned())
    }

}
