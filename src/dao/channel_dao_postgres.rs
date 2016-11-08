use postgres::{Connection, TlsMode};

use dao::{ChannelDao, PostgresUtil};
use error::Error;
use project_types::Channel;

pub struct ChannelDaoPostgres {
    conn: Connection
}


impl ChannelDaoPostgres {
    pub fn new() -> Result<ChannelDaoPostgres, Error> {
        let conn = try!(PostgresUtil::get_connection());
        Ok(ChannelDaoPostgres {
            conn: conn
        })
    }
}

impl ChannelDao for ChannelDaoPostgres {
    /// Fetch a Channel with the given channel id
    fn get_channel(&self, chanid: u32) -> Result<Channel, Error> {
        Err(Error::TodoErr)
    }

    /// Add a channel to the database
    fn add_channel(&self, channel: Channel) -> Result<(), Error> {
        Err(Error::TodoErr)
    }

}
