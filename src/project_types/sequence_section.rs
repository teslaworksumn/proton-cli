
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use rustc_serialize::json;

use error::Error;


#[derive(Clone, Debug, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub struct SequenceSection {
    pub seq_name: String,
    pub index: u32,  // Starts at 1
    pub path: String,
    pub num_frames: u32,
    pub data: Vec<Vec<u8>>, // Row is channel, column is frame    
}

impl SequenceSection {
    /// Write the sequence section to a file
    pub fn write_to_file(&self) -> Result<(), Error> {
        let pretty_json = json::as_pretty_json(&self);
        let section_path = PathBuf::from(&self.path);

        File::create(&section_path)
            .and_then(|mut section_file| write!(section_file, "{}\n", pretty_json))
            .map_err(Error::Io)
    }

}

