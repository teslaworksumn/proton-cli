use project_types::Channel;
use error::Error;
use dao::ChannelDao;

pub struct ChannelDaoPostgres {}

impl ChannelDao for ChannelDaoPostgres {
    fn get_channel(&self, chanid: u32) -> Result<Channel, Error> {
        Err(Error::TodoErr)
    }

}
