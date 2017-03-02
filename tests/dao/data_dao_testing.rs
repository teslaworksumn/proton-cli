extern crate proton_cli;

use proton_cli::dao::DataDao;
use proton_cli::error::Error;


/// Implementation of DataDao for testing purposes. Uses given functions to return values.
/// Functions are boxed so their sizes are known (pointers).
/// The general naming convention used is trait_function_name_fn, for all trait functions.
/// &str references are converted to Strings so we don't have to deal with lifetime headaches (bookdude13 tried on 12/25/16)
#[allow(dead_code)]
pub struct DataDaoTesting {
	pub new_data_default_fn: Box<Fn(u32, Vec<u32>, Vec<u16>) -> Result<(), Error>>,
	pub new_data_fn: Box<Fn(u32, u32, Vec<u16>) -> Result<(), Error>>,	
	pub get_data_fn: Box<Fn(u32, u32) -> Result<Vec<u16>, Error>>,
	pub update_data_fn: Box<Fn(u32, u32, Vec<u16>) -> Result<(), Error>>
}


impl DataDaoTesting {
	/// Creates a new DataDaoTesting struct with all functions set to return Error::TodoErr
    #[allow(dead_code)]
	pub fn new() -> DataDaoTesting {
		DataDaoTesting {
			new_data_default_fn: Box::new(|_, _, _| -> Result<(), Error> { Err(Error::TodoErr) }),
			new_data_fn: Box::new(|_, _, _| -> Result<(), Error>  { Err(Error::TodoErr) }),
			get_data_fn: Box::new(|_, _| -> Result<Vec<u16>, Error> { Err(Error::TodoErr) }),
			update_data_fn: Box::new(|_, _, _| -> Result<(), Error> { Err(Error::TodoErr) }),
		}
	}
}

/// The Dao implementation simply calls the corresponding stored function
impl DataDao for DataDaoTesting {
	fn new_data_default(
        &self,
        seqid: u32,
        chan_ids: Vec<u32>,
        default_data: Vec<u16>
    ) -> Result<(), Error> {
    	(self.new_data_default_fn)(seqid, chan_ids, default_data)
    }

    fn new_data<'a>(
        &'a self,
        seqid: u32,
        chanid: u32,
        new_data: &'a Vec<u16>
    ) -> Result<(), Error> {
    	(self.new_data_fn)(seqid, chanid, new_data.to_owned())
    }
    
    fn get_data(&self, seqid: u32, chanid: u32) -> Result<Vec<u16>, Error> {
    	(self.get_data_fn)(seqid, chanid)
    }
    
    fn update_data<'a>(&'a self, seqid: u32, chanid: u32, new_data: &'a Vec<u16>) -> Result<(), Error> {
    	(self.update_data_fn)(seqid, chanid, new_data.to_owned())
    }
}
