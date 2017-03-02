use error::Error;
use project_types::{Channel, Fixture, Layout, Permission, Project, Section, Sequence, User};

pub trait ChannelDao {
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
    ) -> Result<Channel, Error>;
    
    /// Fetch a Channel with the given channel id
    fn get_channel(&self, chanid: u32) -> Result<Channel, Error>;
    fn get_last_channel(&self, name: &str) -> Result<Channel, Error>;
}

pub trait DataDao {
    fn new_data_default(
        &self,
        seqid: u32,
        chan_ids: Vec<u32>,
        default_data: Vec<u16>
    ) -> Result<(), Error>;
    fn new_data<'a>(
        &'a self,
        seqid: u32,
        chanid: u32,
        new_data: &'a Vec<u16>
    ) -> Result<(), Error>;
    fn get_data(&self, seqid: u32, chanid: u32) -> Result<Vec<u16>, Error>;
    fn update_data<'a>(&'a self, seqid: u32, chanid: u32, new_data: &'a Vec<u16>) -> Result<(), Error>;
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
    fn fixture_exists(&self, fixid: u32) -> Result<bool, Error>;
}

pub trait LayoutDao {
    fn new_layout(&self, name: &str, fixtures: Vec<u32>) -> Result<Layout, Error>;
    fn get_default_layout(&self) -> Result<Layout, Error>;
    fn get_layout(&self, layoutid: u32) -> Result<Layout, Error>;
    fn get_last_layout(&self, name: &str) -> Result<Layout, Error>;
    fn layout_exists(&self, layoutid: u32) -> Result<bool, Error>;
    fn patch_channel(
        &self,
        layoutid: u32,
        channel_internal: u32,
        channel_dmx: u32
    ) -> Result<u64, Error>;
}

pub trait PermissionDao {
    fn add_initial_permission(&self, root_uid: u32) -> Result<(), Error>;
    fn get_all_permissions(&self, uid: u32) -> Result<Vec<Permission>, Error>;
    fn get_permission(&self, permid: u32) -> Result<Permission, Error>;
}

pub trait ProjectDao {
    fn new_project(&self, name: &str, layoutid: u32) -> Result<Project, Error>;
    /// Retrieves the project with the given name. Returns an error if name is invalid or not found.
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
    fn set_layout(&self, seqid: u32, layout_id: u32) -> Result<(), Error>;
    fn sequence_exists(&self, seqid: u32) -> Result<bool, Error>;
    fn get_channel_ids(&self, seqid: u32) -> Result<Vec<u32>, Error>;
}

pub trait UserDao {
    fn add_initial_user(&self, proj_name: &str, private_key: &str, public_key: &str) -> Result<u32, Error>;
    fn add_user(&self, name: &str, private_key: &str, public_key: &str) -> Result<u32, Error>;
    fn get_user(&self, uid: u32) -> Result<User, Error>;
    fn get_user_id(&self, public_key: &str) -> Result<u32, Error>;
}

