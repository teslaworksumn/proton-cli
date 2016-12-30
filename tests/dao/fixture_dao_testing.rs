extern crate proton_cli;

use proton_cli::dao::FixtureDao;
use proton_cli::error::Error;
use proton_cli::project_types::Fixture;


/// Implementation of FixtureDao for testing purposes. Uses given functions to return values.
/// Functions are boxed so their sizes are known (pointers).
/// The general naming convention used is trait_function_name_fn, for all trait functions.
/// &str references are converted to Strings so we don't have to deal with lifetime headaches (bookdude13 tried on 12/25/16)
#[allow(dead_code)]
pub struct FixtureDaoTesting {
	pub new_fixture_fn: Box<Fn(String, (i32, i32, i32), (i32, i32, i32), Vec<u32>) -> Result<Fixture, Error>>,
	pub get_fixture_fn: Box<Fn(u32) -> Result<Fixture, Error>>,
	pub get_last_fixture_fn: Box<Fn(String) -> Result<Fixture, Error>>,
	pub get_num_channels_fn: Box<Fn(u32) -> Result<u32, Error>>,
	pub fixture_exists_fn: Box<Fn(u32) -> Result<bool, Error>>,
}


impl FixtureDaoTesting {
	/// Creates a new FixtureDaoTesting struct with all functions set to return Error::TodoErr
    #[allow(dead_code)]
	pub fn new() -> FixtureDaoTesting {
		FixtureDaoTesting {
			new_fixture_fn: Box::new(|_, _, _, _| -> Result<Fixture, Error> { Err(Error::TodoErr) }),
			get_fixture_fn: Box::new(|_| -> Result<Fixture, Error>  { Err(Error::TodoErr) }),
			get_last_fixture_fn: Box::new(|_| -> Result<Fixture, Error>  { Err(Error::TodoErr) }),
			get_num_channels_fn: Box::new(|_| -> Result<u32, Error>  { Err(Error::TodoErr) }),
			fixture_exists_fn: Box::new(|_| -> Result<bool, Error>  { Err(Error::TodoErr) }),
		}
	}
}

/// The Dao implementation simply calls the corresponding stored function
impl FixtureDao for FixtureDaoTesting {
    fn new_fixture(
        &self, 
        name: &str,
        location: (i32, i32, i32),
        rotation: (i32, i32, i32),
        channels: Vec<u32>
    ) -> Result<Fixture, Error> {
    	(self.new_fixture_fn)(name.to_owned(), location, rotation, channels)
    }
    
    fn get_fixture(&self, fixid: u32) -> Result<Fixture, Error> {
    	(self.get_fixture_fn)(fixid)
    }

    fn get_last_fixture(&self, name: &str) -> Result<Fixture, Error> {
    	(self.get_last_fixture_fn)(name.to_owned())
    }

    fn get_num_channels(&self, fixid: u32) -> Result<u32, Error> {
    	(self.get_num_channels_fn)(fixid)
    }

    fn fixture_exists(&self, fixid: u32) -> Result<bool, Error> {
    	(self.fixture_exists_fn)(fixid)
    }

}
