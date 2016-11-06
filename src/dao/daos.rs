use std::path::Path;

use error::Error;
use project_types::{Channel, Fixture, Layout, Permission, Project, Section, Sequence, User};
use dao;

pub trait ChannelDao {
    fn get_channel(&self, chanid: u32) -> Result<Channel, Error>;
}

pub trait FixtureDao {
    fn get_fixture(&self, fixid: u32) -> Result<Fixture, Error>;
}

pub trait LayoutDao {
    fn get_layout(&self, layoutid: u32) -> Result<Layout, Error>;
}

pub trait PermissionDao {
    fn get_all_permissions(&self, uid: u32) -> Result<Vec<Permission>, Error>;
    fn get_permission(&self, permid: u32) -> Result<Permission, Error>;
}

pub trait SectionDao {
    fn get_section(&self, secid: u32) -> Result<Section, Error>;
}

pub trait SequenceDao {
    fn get_sequence(&self, seqid: u32) -> Result<Sequence, Error>;
}

pub trait UserDao {
    fn get_user(&self, uid: u32) -> Result<User, Error>;
    fn id_user<P: AsRef<Path>>(&self, private_key_path: P) -> Result<u32, Error>;
}

