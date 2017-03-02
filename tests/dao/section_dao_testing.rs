extern crate proton_cli;

use proton_cli::dao::SectionDao;
use proton_cli::error::Error;
use proton_cli::project_types::Section;


/// Implementation of SectionDao for testing purposes. Uses given functions to return values.
/// Functions are boxed so their sizes are known (pointers).
/// The general naming convention used is trait_function_name_fn, for all trait functions.
/// &str references are converted to Strings so we don't have to deal with lifetime headaches (bookdude13 tried on 12/25/16)
#[allow(dead_code)]
pub struct SectionDaoTesting {
	pub get_section_fn: Box<Fn(u32) -> Result<Section, Error>>,
}


impl SectionDaoTesting {
	/// Creates a new SectionDaoTesting struct with all functions set to return Error::TodoErr
	#[allow(dead_code)]
	pub fn new() -> SectionDaoTesting {
		SectionDaoTesting {
			get_section_fn: Box::new(|_| -> Result<Section, Error> { Err(Error::TodoErr) })
		}
	}
}

/// The Dao implementation simply calls the corresponding stored function
impl SectionDao for SectionDaoTesting {
	fn get_section(&self, secid: u32) -> Result<Section, Error> {
		(self.get_section_fn)(secid)
	}

}
