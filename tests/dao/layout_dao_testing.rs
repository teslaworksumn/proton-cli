extern crate proton_cli;

use proton_cli::dao::LayoutDao;
use proton_cli::error::Error;
use proton_cli::project_types::Layout;


pub struct LayoutDaoTesting {
	pub new_layout_fn: Box<Fn(String, Vec<u32>) -> Result<Layout, Error>>,
	pub default_layout_fn: Box<Fn() -> Result<Layout, Error>>,
	pub get_layout_fn: Box<Fn(u32) -> Result<Layout, Error>>,
	pub get_last_layout_fn: Box<Fn(String) -> Result<Layout, Error>>,
	pub layout_exists_fn: Box<Fn(u32) -> Result<bool, Error>>,
	pub patch_channel_fn: Box<Fn(u32, u32, u32) -> Result<u64, Error>>,
}


impl LayoutDaoTesting {
	pub fn new() -> LayoutDaoTesting {
		LayoutDaoTesting {
			new_layout_fn: Box::new(|_, _| -> Result<Layout, Error> { Err(Error::TodoErr) }),
			default_layout_fn: Box::new(|| -> Result<Layout, Error>  { Err(Error::TodoErr) }),
			get_layout_fn: Box::new(|_| -> Result<Layout, Error> { Err(Error::TodoErr) }),
			get_last_layout_fn: Box::new(|_| -> Result<Layout, Error> { Err(Error::TodoErr) }),
			layout_exists_fn: Box::new(|_| -> Result<bool, Error> { Err(Error::TodoErr) }),
			patch_channel_fn: Box::new(|_, _, _| -> Result<u64, Error> { Err(Error::TodoErr) })
		}
	}
}

impl LayoutDao for LayoutDaoTesting {
	fn new_layout(&self, name: &str, fixtures: Vec<u32>) -> Result<Layout, Error> {
		(self.new_layout_fn)(name.to_owned(), fixtures)
	}
    
	fn get_default_layout(&self) -> Result<Layout, Error> {
		(self.default_layout_fn)()
	}

    fn get_layout(&self, layoutid: u32) -> Result<Layout, Error> {
    	(self.get_layout_fn)(layoutid)
    }

    fn get_last_layout(&self, name: &str) -> Result<Layout, Error> {
    	(self.get_last_layout_fn)(name.to_owned())
    }

    fn layout_exists(&self, layoutid: u32) -> Result<bool, Error> {
    	(self.layout_exists_fn)(layoutid)
    }

    fn patch_channel(
        &self,
        layoutid: u32,
        channel_internal: u32,
        channel_dmx: u32
    ) -> Result<u64, Error> {
    	(self.patch_channel_fn)(layoutid, channel_internal, channel_dmx)
    }

}
