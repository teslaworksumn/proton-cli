use dao::{ChannelDao, ChannelDaoPostgres};
use error::Error;
use project_types::Channel;


impl ChannelDao for ChannelDaoPostgres {
    /// Fetch a Channel with the given channel id
    fn get_channel(&self, chanid: u32) -> Result<Channel, Error> {
        let query = "SELECT name,primary_num,secondary_num,color,channel_internal,channel_dmx, \
        location_x,location_y,location_z,rotation_a,rotation_b,rotation_c \
        FROM channels WHERE chanid = $1";
        let results = try!(
            self.conn.query(query, &[&(chanid as i32)])
            .map_err(Error::Postgres));
        match results.len() {
            0 => Err(Error::ChannelNotFound(chanid)),
            1 => {
                let row = results.get(0);
                let name: String = row.get(0);
                let primary_num: Option<i32> = row.get(1);
                let secondary_num: Option<i32> = row.get(2);
                let color: String = row.get(3);
                let channel_internal: i32 = row.get(4);
                let channel_dmx: i32 = row.get(5);
                let location_x: Option<i32> = row.get(6);
                let location_y: Option<i32> = row.get(7);
                let location_z: Option<i32> = row.get(8);
                let rotation_a: Option<i32> = row.get(9);
                let rotation_b: Option<i32> = row.get(10);
                let rotation_c: Option<i32> = row.get(11);
                Ok(Channel {
                    chanid: chanid,
                    name: name,
                    numbers: (primary_num.map(|pnum| pnum as u32), secondary_num.map(|snum| snum as u32)),
                    color: color,
                    channel_internal: channel_internal as u32,
                    channel_dmx: channel_dmx as u32,
                    location: (location_x, location_y, location_z),
                    rotation: (rotation_a, rotation_b, rotation_c)
                })
            },
            x => Err(Error::InvalidNumResults(x)),
        }
    }

    /// Add a channel to the database
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
        let statement = "INSERT INTO channels (name,primary_num,secondary_num,\
            color,channel_internal,channel_dmx,location_x,location_y,location_z,\
            rotation_a,rotation_b,rotation_c) \
            VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12)";
        let _ = try!(
            self.conn.execute(
                statement,
                &[
                    &name.to_owned(),
                    &(primary_num.map(|pnum| pnum as i32)),
                    &(secondary_num.map(|snum| snum as i32)),
                    &color.to_owned(),
                    &(channel_internal as i32),
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
        let query = "SELECT chanid,primary_num,secondary_num,color,channel_internal,channel_dmx,\
        location_x,location_y,location_z,rotation_a,rotation_b,rotation_c FROM channels \
        WHERE name = $1 ORDER BY chanid DESC";
        let results = try!(
            self.conn.query(query, &[&name.to_owned()])
            .map_err(Error::Postgres));
        if results.len() == 0 {
            return Err(Error::ChannelNotFound(0));
        }
        // First row has highest chanid
        let row = results.get(0);
        let chanid: i32 = row.get(0);
        let primary_num: Option<i32> = row.get(1);
        let secondary_num: Option<i32> = row.get(2);
        let color: String = row.get(3);
        let channel_internal: i32 = row.get(4);
        let channel_dmx: i32 = row.get(5);
        let location_x: Option<i32> = row.get(6);
        let location_y: Option<i32> = row.get(7);
        let location_z: Option<i32> = row.get(8);
        let rotation_a: Option<i32> = row.get(9);
        let rotation_b: Option<i32> = row.get(10);
        let rotation_c: Option<i32> = row.get(11);
        Ok(Channel {
            chanid: chanid as u32,
            name: name.to_owned(),
            numbers: (primary_num.map(|pnum| pnum as u32), secondary_num.map(|snum| snum as u32)),
            color: color,
            channel_internal: channel_internal as u32,
            channel_dmx: channel_dmx as u32,
            location: (location_x, location_y, location_z),
            rotation: (rotation_a, rotation_b, rotation_c)
        })
    }
}
