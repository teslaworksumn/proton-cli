use std::collections::HashMap;

use dao::{ChannelDao, FixtureDao};
use error::Error;
use project_types::{Channel, Fixture};

#[derive(Debug, RustcDecodable)]
#[allow(non_snake_case)]
pub struct FileLayout {
    pub layoutName: String,
    pub channels: Vec<FileLayoutRow>,
}

#[derive(Debug, RustcDecodable)]
#[allow(non_snake_case)]
pub struct FileLayoutRow {
    pub internalChannel: u32,
    pub dmxChannel: u32,
    pub fixtureName: String,
    pub channelName: String,
    pub color: String,
    pub num_primary: Option<u32>, // Default is 0
    pub num_secondary: Option<u32>, // Default is 0
    pub location: String, // Default is "0,0,0"
    pub rotation: String, // Default is "0,0,0"
}

impl FileLayout {

    fn layout_str_to_i32<'a>(s: &'a str, err_msg: &'a str) -> Result<Option<i32>, Error> {
        match s.is_empty() {
            true => Ok(None::<i32>),
            false => s.parse::<i32>()
                .map(|s_i32| Some(s_i32))
                .map_err(|_| Error::InvalidLayout(String::from(err_msg)))
        }
    }

    fn layout_get_i32_tuple(s: &str) -> Result<(Option<i32>, Option<i32>, Option<i32>), Error> {
        let parts = s.trim_matches(',').split(',').collect::<Vec<&str>>();
        if parts.len() != 3 {
            return Err(Error::InvalidLayout(String::from("Locations must be of the form x,y,z")))
        }
        let x = try!(FileLayout::layout_str_to_i32(parts[0], "first element is not an i32"));
        let y = try!(FileLayout::layout_str_to_i32(parts[1], "second element is not an i32"));
        let z = try!(FileLayout::layout_str_to_i32(parts[2], "third element is not an i32"));
        Ok((x,y,z))
    }

    /// Check that all channels are valid
    pub fn validate(&self) -> Result<(), Error> {

        // Validate layout name not too long and only alphanumerics
        if self.layoutName.len() > 64 {
            return Err(Error::InvalidLayout(String::from("Layout name cannot be longer than 64 characters")))
        }
        if !self.layoutName.chars().all(char::is_alphanumeric) {
            return Err(Error::InvalidLayout(String::from("Layout name has to be alphanumeric: ") + &self.layoutName))   
        }

        for channel in &self.channels {

            // Make sure internal channel > 0 (indexed same as DMX)
            if channel.internalChannel < 1 {
                return Err(Error::InvalidLayout(String::from("Internal channels start at 1, not 0")))
            }

            // Make sure DMX > 0
            if channel.dmxChannel < 1 {
                return Err(Error::InvalidLayout(String::from("DMX channels start at 1, not 0")))
            }
            
            // Validate locations and each piece
            let _ = try!(FileLayout::layout_get_i32_tuple(&channel.location));

            // Validate rotations and each piece
            let _ = try!(FileLayout::layout_get_i32_tuple(&channel.rotation));

            // Validate channel name not too long and only alphanumerics or spaces
            if channel.channelName.len() > 40 {
                return Err(Error::InvalidLayout(String::from("Channel name cannot be longer than 40 characters")));
            }
            if !channel.channelName.chars().all(|c| c.is_alphanumeric() || c == ' ') {
                return Err(Error::InvalidLayout(String::from("Channel name has to be alphanumeric: ") + &channel.channelName));
            }

            // Validate name not too long and only alphanumerics or spaces
            if channel.fixtureName.len() > 40 {
                return Err(Error::InvalidLayout(String::from("Fixture name cannot be longer than 40 characters")))
            }
            if !channel.fixtureName.chars().all(|c| c.is_alphanumeric() || c == ' ') {
                return Err(Error::InvalidLayout(String::from("Fixture name has to be alphanumeric: ") + &channel.fixtureName))   
            }

            // Validate color not too long and only alphanumerics or spaces
            if channel.color.len() > 16 {
                return Err(Error::InvalidLayout(String::from("Color cannot be longer than 16 characters")))
            }
            if !channel.color.chars().all(|c| c.is_alphanumeric() || c == ' ') {
                return Err(Error::InvalidLayout(String::from("Color has to be alphanumeric")))   
            }
        }
        Ok(())
    }

    pub fn create_new_parts<CD: ChannelDao, FD: FixtureDao>(
        &self,
        chan_dao: &CD,
        fix_dao: &FD
    ) -> Result<(Vec<Channel>, Vec<Fixture>), Error> {
    
        let mut channels = Vec::new();
        let mut fixture_names = HashMap::new();
        // Create channels and add to vec. Place ids in fixture buckets
        // Ignore channels with name of "Spare" or "X"
        for c in &self.channels {
            if c.channelName != "Spare" && c.channelName != "X" {
                let location = try!(FileLayout::layout_get_i32_tuple(&c.location));
                let rotation = try!(FileLayout::layout_get_i32_tuple(&c.rotation));
                let channel = try!(chan_dao.new_channel(
                    &c.channelName,
                    c.num_primary,
                    c.num_secondary,
                    &c.color,
                    c.internalChannel,
                    c.dmxChannel,
                    location,
                    rotation));
                let fix_name = channel.name.clone();
                if !fixture_names.contains_key(&fix_name) {
                    fixture_names.insert(fix_name, vec![channel.chanid]);
                } else {
                    fixture_names.get_mut(&fix_name).unwrap().push(channel.chanid);
                }
                channels.push(channel);
            }
        }

        // Create fixtures
        let mut fixtures = Vec::new();
        for (fix_name, fix_chan_ids) in &fixture_names {
            // TODO: Calculate center and width/height of fixture
            let fixture = try!(fix_dao.new_fixture(
                fix_name,
                (0,0,0),
                (0,0,0),
                fix_chan_ids.to_owned()
            ));
            fixtures.push(fixture);
        }

        Ok((channels, fixtures))
    }
}
