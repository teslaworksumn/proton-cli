use dao::{ChannelDao, ChannelDaoPostgres};
use error::Error;
use project_types::Channel;


impl ChannelDao for ChannelDaoPostgres {
    /// Fetch a Channel with the given channel id
    fn get_channel(&self, chanid: u32) -> Result<Channel, Error> {
        Err(Error::TodoErr)
    }

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
    ) -> Result<Channel, Error> {
        let statement = "INSERT INTO channels (name,primary_num,secondary_num,\
            color,channel_dmx,location_x,location_y,location_z,rotation_a,rotation_b,rotation_c) \
            VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11)";
        let _ = try!(
            self.conn.execute(
                statement,
                &[
                    &name.to_owned(),
                    &(primary_num as i32),
                    &(secondary_num as i32),
                    &color.to_owned(),
                    &(channel_dmx as i32),
                    &location.0,
                    &location.1,
                    &location.2,
                    &rotation.0,
                    &rotation.1,
                    &rotation.2
                ])
            .map_err(Error::Postgres));
        // Get the most recently added channel with the given name
        let channel = try!(self.get_last_channel(name));
        Ok(channel)
    }

    fn get_last_channel(&self, name: &str) -> Result<Channel, Error> {
        let query = "SELECT chanid, primary_num, secondary_num, color, channel_dmx, location_x, \
        location_y, location_z, rotation_a, rotation_b, rotation_c FROM channels WHERE name = $1 \
        ORDER BY chanid DESC";
        let results = try!(
            self.conn.query(query, &[&name.to_owned()])
            .map_err(Error::Postgres));
        if results.len() == 0 {
            return Err(Error::ChannelNotFound(0));
        }
        // First row has highest chanid
        let row = results.get(0);
        let chanid: i32 = row.get(0);
        let primary_num: i32 = row.get(1);
        let secondary_num: i32 = row.get(2);
        let color: String = row.get(3);
        let channel_dmx: i32 = row.get(4);
        let location_x: i32 = row.get(5);
        let location_y: i32 = row.get(6);
        let location_z: i32 = row.get(7);
        let rotation_a: i32 = row.get(8);
        let rotation_b: i32 = row.get(9);
        let rotation_c: i32 = row.get(10);
        Ok(Channel {
            chanid: chanid as u32,
            name: name.to_owned(),
            numbers: (primary_num as u32, secondary_num as u32),
            color: color,
            channel_dmx: channel_dmx as u32,
            location: (location_x, location_y, location_z),
            rotation: (rotation_a, rotation_b, rotation_c)
        })
    }


}
