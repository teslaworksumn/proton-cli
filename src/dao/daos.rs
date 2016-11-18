use std::path::Path;

use error::Error;
use project_types::{Channel, Fixture, Layout, Permission, Project, Section, Sequence, User};
use dao;

pub trait ChannelDao {
    /// Add a channel to the database
    fn add_channel(&self, channel: Channel) -> Result<(), Error>;
    /// Fetch a Channel with the given channel id
    fn get_channel(&self, chanid: u32) -> Result<Channel, Error>;
}

pub trait FixtureDao {
    fn get_fixture(&self, fixid: u32) -> Result<Fixture, Error>;   
    fn get_num_channels(&self, fixid: u32) -> Result<u32, Error>;
}

pub trait LayoutDao {
    fn get_default_layout(&self) -> Result<Layout, Error>;
    fn get_layout(&self, layoutid: u32) -> Result<Layout, Error>;
}

pub trait PermissionDao {
    fn add_initial_permission(&self, root_uid: u32) -> Result<(), Error>;
    fn get_all_permissions(&self, uid: u32) -> Result<Vec<Permission>, Error>;
    fn get_permission(&self, permid: u32) -> Result<Permission, Error>;
}

pub trait SectionDao {
    fn get_section(&self, secid: u32) -> Result<Section, Error>;
}

pub trait SequenceDao {
    fn get_sequence(&self, seqid: u32) -> Result<Sequence, Error>;
    fn new_sequence(&self, sequence: &Sequence) -> Result<(), Error>;
}

pub trait UserDao {
    fn add_initial_user(&self, private_key: &str, public_key: &str) -> Result<u32, Error>;
    fn add_user(&self, name: &str, private_key: &str, public_key: &str) -> Result<u32, Error>;
    fn get_user(&self, uid: u32) -> Result<User, Error>;
    fn get_user_id(&self, public_key: &str) -> Result<u32, Error>;
}

