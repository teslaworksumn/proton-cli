extern crate proton_cli;

use proton_cli::dao::PermissionDao;
use proton_cli::error::Error;
use proton_cli::project_types::Permission;


/// Implementation of PermissionDao for testing purposes. Uses given functions to return values.
/// Functions are boxed so their sizes are known (pointers).
/// The general naming convention used is trait_function_name_fn, for all trait functions.
/// &str references are converted to Strings so we don't have to deal with lifetime headaches (bookdude13 tried on 12/25/16)
#[allow(dead_code)]
pub struct PermissionDaoTesting {
	pub add_initial_permission_fn: Box<Fn(u32) -> Result<(), Error>>,
	pub get_all_permissions_fn: Box<Fn(u32) -> Result<Vec<Permission>, Error>>,
	pub get_permission_fn: Box<Fn(u32) -> Result<Permission, Error>>,
}


impl PermissionDaoTesting {
	/// Creates a new PermissionDaoTesting struct with all functions set to return Error::TodoErr
	#[allow(dead_code)]
	pub fn new() -> PermissionDaoTesting {
		PermissionDaoTesting {
			add_initial_permission_fn: Box::new(|_| -> Result<(), Error> { Err(Error::TodoErr) }),
			get_all_permissions_fn: Box::new(|_| -> Result<Vec<Permission>, Error>  { Err(Error::TodoErr) }),
			get_permission_fn: Box::new(|_| -> Result<Permission, Error> { Err(Error::TodoErr) }),
		}
	}
}

/// The Dao implementation simply calls the corresponding stored function
impl PermissionDao for PermissionDaoTesting {
	fn add_initial_permission(&self, root_uid: u32) -> Result<(), Error> {
		(self.add_initial_permission_fn)(root_uid)
	}

    fn get_all_permissions(&self, uid: u32) -> Result<Vec<Permission>, Error> {
    	(self.get_all_permissions_fn)(uid)
    }

    fn get_permission(&self, permid: u32) -> Result<Permission, Error> {
    	(self.get_permission_fn)(permid)
    }

}
