extern crate proton_cli;

use proton_cli::dao::SequenceDao;
use proton_cli::error::Error;
use proton_cli::project_types::Sequence;


/// Implementation of SequenceDao for testing purposes. Uses given functions to return values.
/// Functions are boxed so their sizes are known (pointers).
/// The general naming convention used is trait_function_name_fn, for all trait functions.
/// &str references are converted to Strings so we don't have to deal with lifetime headaches (bookdude13 tried on 12/25/16)
#[allow(dead_code)]
pub struct SequenceDaoTesting {
	pub get_sequence_fn: Box<Fn(u32) -> Result<Sequence, Error>>,
	pub get_last_sequence_fn: Box<Fn(String) -> Result<Sequence, Error>>,
	pub new_sequence_fn: Box<Fn(Sequence) -> Result<Sequence, Error>>,
	pub set_layout_fn: Box<Fn(u32, u32) -> Result<(), Error>>,
	pub sequence_exists_fn: Box<Fn(u32) -> Result<bool, Error>>,
	pub get_channel_ids_fn: Box<Fn(u32) -> Result<Vec<u32>, Error>>,
}


impl SequenceDaoTesting {
	/// Creates a new SequenceDaoTesting struct with all functions set to return Error::TodoErr
	#[allow(dead_code)]
	pub fn new() -> SequenceDaoTesting {
		SequenceDaoTesting {
			get_sequence_fn: Box::new(|_| -> Result<Sequence, Error> { Err(Error::TodoErr) }),
			get_last_sequence_fn: Box::new(|_| -> Result<Sequence, Error>  { Err(Error::TodoErr) }),
			new_sequence_fn: Box::new(|_| -> Result<Sequence, Error> { Err(Error::TodoErr) }),
			set_layout_fn: Box::new(|_, _| -> Result<(), Error> { Err(Error::TodoErr) }),
			sequence_exists_fn: Box::new(|_| -> Result<bool, Error> { Err(Error::TodoErr) }),
			get_channel_ids_fn: Box::new(|_| -> Result<Vec<u32>, Error> { Err(Error::TodoErr) })
		}
	}
}

/// The Dao implementation simply calls the corresponding stored function
impl SequenceDao for SequenceDaoTesting {
	fn get_sequence(&self, seqid: u32) -> Result<Sequence, Error> {
		(self.get_sequence_fn)(seqid)
	}

    fn get_last_sequence(&self, name: &str) -> Result<Sequence, Error> {
    	(self.get_last_sequence_fn)(name.to_owned())
    }

    fn new_sequence(&self, sequence: &Sequence) -> Result<Sequence, Error> {
    	(self.new_sequence_fn)(sequence.to_owned())
    }

    fn set_layout(&self, seqid: u32, layout_id: u32) -> Result<(), Error> {
    	(self.set_layout_fn)(seqid, layout_id)
    }

    fn sequence_exists(&self, seqid: u32) -> Result<bool, Error> {
    	(self.sequence_exists_fn)(seqid)
    }

    fn get_channel_ids(&self, seqid: u32) -> Result<Vec<u32>, Error> {
    	(self.get_channel_ids_fn)(seqid)
    }

}
