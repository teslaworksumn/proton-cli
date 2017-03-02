use error::Error;
use project_types::{Channel, Fixture, Layout, Permission, Project, Section, Sequence, User};


/// Handles metadata related to channels
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

    /// Get the last channel added with the given name
    fn get_last_channel(&self, name: &str) -> Result<Channel, Error>;
}

/// Handles the raw output data for specific channel/sequence pairs
pub trait DataDao {
    /// Add a data entry for all given channels with default data
    fn new_data_default(
        &self,
        seqid: u32,
        chan_ids: Vec<u32>,
        default_data: Vec<u16>
    ) -> Result<(), Error>;

    /// Add a new data entry for a given sequence and channel with provided data
    fn new_data<'a>(
        &'a self,
        seqid: u32,
        chanid: u32,
        new_data: &'a Vec<u16>
    ) -> Result<(), Error>;

    /// Retrieve the data for a given sequence and channel
    fn get_data(&self, seqid: u32, chanid: u32) -> Result<Vec<u16>, Error>;

    /// Update a sequence's channel's data
    fn update_data<'a>(&'a self, seqid: u32, chanid: u32, new_data: &'a Vec<u16>) -> Result<(), Error>;
}

/// Handles metadata related to fixtures
pub trait FixtureDao {
    /// Creates a new fixture made up of some channels
    fn new_fixture(
        &self, 
        name: &str,
        location: (i32, i32, i32),
        rotation: (i32, i32, i32),
        channels: Vec<u32>
    ) -> Result<Fixture, Error>;

    /// Retrieves and returns a fixture based on its id
    fn get_fixture(&self, fixid: u32) -> Result<Fixture, Error>;

    /// Retrieves and returns the last fixture added with the given name
    fn get_last_fixture(&self, name: &str) -> Result<Fixture, Error>;

    /// Retrieves and returns the number of channels a fixture has
    fn get_num_channels(&self, fixid: u32) -> Result<u32, Error>;

    /// Returns true if the fixture exists, false otherwise
    fn fixture_exists(&self, fixid: u32) -> Result<bool, Error>;
}

/// Handles metadata related to layouts
pub trait LayoutDao {
    /// Create a new layout with the given fixtures
    fn new_layout(&self, name: &str, fixtures: Vec<u32>) -> Result<Layout, Error>;

    /// Retrieve and return the default layout
    fn get_default_layout(&self) -> Result<Layout, Error>;

    /// Retrieve and return a layout
    fn get_layout(&self, layoutid: u32) -> Result<Layout, Error>;

    /// Retrieve and return the last layout created with the given name
    fn get_last_layout(&self, name: &str) -> Result<Layout, Error>;

    /// Returns true if the layout exists, false otherwise
    fn layout_exists(&self, layoutid: u32) -> Result<bool, Error>;

    /// Patch a channel (change a channel's dmx output channel)
    fn patch_channel(
        &self,
        layoutid: u32,
        channel_internal: u32,
        channel_dmx: u32
    ) -> Result<u64, Error>;
}

/// [INCOMPLETE] Handles data related to permissions
pub trait PermissionDao {
    fn add_initial_permission(&self, root_uid: u32) -> Result<(), Error>;
    fn get_all_permissions(&self, uid: u32) -> Result<Vec<Permission>, Error>;
    fn get_permission(&self, permid: u32) -> Result<Permission, Error>;
}

/// Handles project metadata
pub trait ProjectDao {
    /// Create a new project
    fn new_project(&self, name: &str, layoutid: u32) -> Result<Project, Error>;

    /// Retrieves the project with the given name. Returns an error if name is invalid or not found.
    fn get_project(&self, name: &str) -> Result<Project, Error>;

    /// Update a project's metadata
    fn update_project(&self, new_project: Project) -> Result<(), Error>;
}

/// Handles metadata about sections of sequences
pub trait SectionDao {
    /// Retrieve and return a sequence section
    fn get_section(&self, secid: u32) -> Result<Section, Error>;
}

/// Handles metadata related to sequences
pub trait SequenceDao {
    /// Retrieves and returns a sequence
    fn get_sequence(&self, seqid: u32) -> Result<Sequence, Error>;

    /// Retrieves and returns the last sequence added with the given name
    fn get_last_sequence(&self, name: &str) -> Result<Sequence, Error>;

    /// Creates a new sequence
    fn new_sequence(&self, sequence: &Sequence) -> Result<Sequence, Error>;

    /// Sets a sequence's layout
    fn set_layout(&self, seqid: u32, layout_id: u32) -> Result<(), Error>;

    /// Returns true if the sequence exists, false otherwise
    fn sequence_exists(&self, seqid: u32) -> Result<bool, Error>;

    /// Retrieves and returns a vector of all channels in a sequence
    fn get_channel_ids(&self, seqid: u32) -> Result<Vec<u32>, Error>;
}

/// Handles user data
pub trait UserDao {
    /// Creates a project's initial root user
    fn add_initial_user(&self, proj_name: &str, private_key: &str, public_key: &str) -> Result<u32, Error>;

    /// Creates a new user
    fn add_user(&self, name: &str, private_key: &str, public_key: &str) -> Result<u32, Error>;

    /// Retrieves and returns a user
    fn get_user(&self, uid: u32) -> Result<User, Error>;

    /// Identifies a user based on their public key
    fn get_user_id(&self, public_key: &str) -> Result<u32, Error>;
}
