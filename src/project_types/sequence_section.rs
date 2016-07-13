
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use rustc_serialize::json;

use error::Error;
use project_types::User;


#[derive(Clone, Debug, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub struct SequenceSection {
    pub seq_name: String,
    pub index: u32,
    pub num_frames: u32,
    pub editor: Option<User>,
    pub data: Vec<Vec<u8>>, // Row is channel, column is frame    
}

impl SequenceSection {
    /// Write the sequence section to a file
    pub fn write_to_file(&self, seq_directory_name: &str) -> Result<(), Error> {
        let pretty_json = json::as_pretty_json(&self);
        let section_path = &self.get_path(&seq_directory_name);

        File::create(&section_path)
            .and_then(|mut section_file| write!(section_file, "{}\n", pretty_json))
            .map_err(Error::Io)
    }

    /// Get the path to this specific section, starting with the sequence directory
    /// E.g. sequence/sequence_section1.json
    /// Assumes the current directory is the project directory
    fn get_path(&self, directory_name: &str) -> PathBuf {

        let mut filename = String::new();
        filename.push_str(&self.seq_name);
        filename.push_str("_section");
        filename.push_str(&self.index.to_string());

        let mut section_path = PathBuf::from(&directory_name);
        section_path.push(&filename);
        
        section_path
    }
}

