use error::Error;
use project_types::{Channel, Fixture, Layout};

#[derive(Debug, RustcDecodable)]
pub struct FileLayout {
    pub channels: Vec<FileLayoutRow>,
}

#[derive(Debug, RustcDecodable)]
pub struct FileLayoutRow {
    pub dmxChannel: u32,
    pub name: String,
    pub color: String,
    pub num_primary: u32, // Default is 0
    pub num_secondary: u32, // Default is 0
    pub location: String, // Default is "0,0,0"
    pub rotation: String, // Default is "0,0,0"
}

impl FileLayout {

    fn layout_str_to_i32<'a>(s: &'a str, err_msg: &'a str) -> Result<i32, Error> {
        s.parse::<i32>().map_err(|_| Error::InvalidLayout(String::from(err_msg)))
    }

    pub fn as_layout(&self) -> Result<(Layout, Vec<Fixture>, Vec<Channel>), Error> {
        for channel in &self.channels {
            // Validate locations and break into pieces
            let locations = channel.location.trim_matches(',').split(',').collect::<Vec<&str>>();
            if locations.len() != 3 {
                return Err(Error::InvalidLayout(String::from("Locations must be of the form x,y,z")))
            }
            let x = try!(FileLayout::layout_str_to_i32(locations[0], "x is not an i32"));
            let y = try!(FileLayout::layout_str_to_i32(locations[1], "y is not an i32"));
            let z = try!(FileLayout::layout_str_to_i32(locations[2], "z is not an i32"));

            // Validate rotations and break into pieces
            let rotations = channel.rotation.trim_matches(',').split(',').collect::<Vec<&str>>();
            if rotations.len() != 3 {
                return Err(Error::InvalidLayout(String::from("Rotations must be of the form a,b,c")))
            }
            let a = try!(FileLayout::layout_str_to_i32(rotations[0], "a is not an i32"));
            let b = try!(FileLayout::layout_str_to_i32(rotations[1], "b is not an i32"));
            let c = try!(FileLayout::layout_str_to_i32(rotations[2], "c is not an i32"));
        }
        Err(Error::TodoErr)
    }
}


