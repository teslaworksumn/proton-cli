use error::Error;
use project_types::{Channel, Fixture, Layout, Permission, Project, Section, Sequence, User};

pub trait ChannelDao {
    /// Add a channel to the database
    fn new_channel(
        &self,
        name: &str,
        primary_num: u32,
        secondary_num: u32,
        color: &str,
        channel_dmx: u32,
        location: (i32, i32, i32),
        rotation: (i32, i32, i32)
    ) -> Result<Channel, Error>;
    
    /// Fetch a Channel with the given channel id
    fn get_channel(&self, chanid: u32) -> Result<Channel, Error>;
    fn get_last_channel(&self, name: &str) -> Result<Channel, Error>;
}

pub trait FixtureDao {
    fn new_fixture(
        &self, 
        name: &str,
        location: (i32, i32, i32),
        rotation: (i32, i32, i32),
        channels: Vec<u32>
    ) -> Result<Fixture, Error>;
    fn get_fixture(&self, fixid: u32) -> Result<Fixture, Error>;
    fn get_last_fixture(&self, name: &str) -> Result<Fixture, Error>;
    fn get_num_channels(&self, fixid: u32) -> Result<u32, Error>;
}

pub trait LayoutDao {
    fn new_layout(&self, name: &str, fixtures: Vec<u32>) -> Result<Layout, Error>;
    fn get_default_layout(&self) -> Result<Layout, Error>;
    fn get_layout(&self, layoutid: u32) -> Result<Layout, Error>;
    fn get_last_layout(&self, name: &str) -> Result<Layout, Error>;
}

pub trait PermissionDao {
    fn add_initial_permission(&self, root_uid: u32) -> Result<(), Error>;
    fn get_all_permissions(&self, uid: u32) -> Result<Vec<Permission>, Error>;
    fn get_permission(&self, permid: u32) -> Result<Permission, Error>;
}

pub trait ProjectDao {
    fn new_project(&self, name: &str, layoutid: u32) -> Result<Project, Error>;
    fn get_project(&self, name: &str) -> Result<Project, Error>;
    fn update_project(&self, new_project: Project) -> Result<(), Error>;
}

pub trait SectionDao {
    fn get_section(&self, secid: u32) -> Result<Section, Error>;
}

pub trait SequenceDao {
    fn get_sequence(&self, seqid: u32) -> Result<Sequence, Error>;
    fn get_last_sequence(&self, name: &str) -> Result<Sequence, Error>;
    fn new_sequence(&self, sequence: &Sequence) -> Result<Sequence, Error>;
}

pub trait UserDao {
    fn add_initial_user(&self, proj_name: &str, private_key: &str, public_key: &str) -> Result<u32, Error>;
    fn add_user(&self, name: &str, private_key: &str, public_key: &str) -> Result<u32, Error>;
    fn get_user(&self, uid: u32) -> Result<User, Error>;
    fn get_user_id(&self, public_key: &str) -> Result<u32, Error>;
}

