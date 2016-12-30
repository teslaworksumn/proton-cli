extern crate proton_cli;

use proton_cli::dao::ChannelDao;
use proton_cli::error::Error;
use proton_cli::project_types::Channel;


/// Implementation of ChannelDao for testing purposes. Uses given functions to return values.
/// Functions are boxed so their sizes are known (pointers).
/// The general naming convention used is trait_function_name_fn, for all trait functions.
/// &str references are converted to Strings so we don't have to deal with lifetime headaches (bookdude13 tried on 12/25/16)
#[allow(dead_code)]
pub struct ChannelDaoTesting {
	pub new_channel_fn: Box<Fn(
		String,
        Option<u32>,
        Option<u32>,
        String,
        u32,
        u32,
        (Option<i32>, Option<i32>, Option<i32>),
        (Option<i32>, Option<i32>, Option<i32>)) -> Result<Channel, Error>>,
	pub get_channel_fn: Box<Fn(u32) -> Result<Channel, Error>>,
	pub get_last_channel_fn: Box<Fn(String) -> Result<Channel, Error>>,
}


impl ChannelDaoTesting {
	/// Creates a new ChannelDaoTesting struct with all functions set to return Error::TodoErr
    #[allow(dead_code)]
	pub fn new() -> ChannelDaoTesting {
		ChannelDaoTesting {
			new_channel_fn: Box::new(|_, _, _, _, _, _, _, _| -> Result<Channel, Error> { Err(Error::TodoErr) }),
			get_channel_fn: Box::new(|_| -> Result<Channel, Error> { Err(Error::TodoErr) }),
			get_last_channel_fn: Box::new(|_| -> Result<Channel, Error> { Err(Error::TodoErr) })
		}
	}
}

/// The Dao implementation simply calls the corresponding stored function
impl ChannelDao for ChannelDaoTesting {
	/// Add a channel
    fn new_channel(
        &self,
        name: &str,
        primary_num: Option<u32>,
        secondary_num: Option<u32>,
        color: &str,
        channel_internal: u32,
        channel_dmx: u32,
        location: (Option<i32>, Option<i32>, Option<i32>),
        rotation: (Option<i32>, Option<i32>, Option<i32>)
    ) -> Result<Channel, Error> {
    	(self.new_channel_fn)(
    		name.to_owned(),
    		primary_num,
    		secondary_num,
    		color.to_owned(),
    		channel_internal,
    		channel_dmx,
    		location,
    		rotation)
    }
    
    /// Fetch a Channel with the given channel id
    fn get_channel(&self, chanid: u32) -> Result<Channel, Error> {
    	(self.get_channel_fn)(chanid)
    }

    fn get_last_channel(&self, name: &str) -> Result<Channel, Error> {
    	(self.get_last_channel_fn)(name.to_owned())
    }
}
