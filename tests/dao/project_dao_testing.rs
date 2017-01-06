extern crate proton_cli;

use proton_cli::dao::ProjectDao;
use proton_cli::error::Error;
use proton_cli::project_types::Project;


/// Implementation of ProjectDao for testing purposes. Uses given functions to return values.
/// Functions are boxed so their sizes are known (pointers).
/// The general naming convention used is trait_function_name_fn, for all trait functions.
/// &str references are converted to Strings so we don't have to deal with lifetime headaches (bookdude13 tried on 12/25/16)
#[allow(dead_code)]
pub struct ProjectDaoTesting {
	pub new_project_fn: Box<Fn(String, u32) -> Result<Project, Error>>,
	pub get_project_fn: Box<Fn(String) -> Result<Project, Error>>,
	pub update_project_fn: Box<Fn(Project) -> Result<(), Error>>,
}


impl ProjectDaoTesting {
	/// Creates a new ProjectDaoTesting struct with all functions set to return Error::TodoErr
	#[allow(dead_code)]
	pub fn new() -> ProjectDaoTesting {
		ProjectDaoTesting {
			new_project_fn: Box::new(|_, _| -> Result<Project, Error> { Err(Error::TodoErr) }),
			get_project_fn: Box::new(|_| -> Result<Project, Error> { Err(Error::TodoErr) }),
			update_project_fn: Box::new(|_| -> Result<(), Error> { Err(Error::TodoErr) })
		}
	}
}

/// The Dao implementation simply calls the corresponding stored function
impl ProjectDao for ProjectDaoTesting {
	fn new_project(&self, name: &str, layoutid: u32) -> Result<Project, Error> {
		(self.new_project_fn)(name.to_owned(), layoutid)
	}

    fn get_project(&self, name: &str) -> Result<Project, Error> {
    	(self.get_project_fn)(name.to_owned())
    }

    fn update_project(&self, new_project: Project) -> Result<(), Error> {
    	(self.update_project_fn)(new_project)
    }
}
