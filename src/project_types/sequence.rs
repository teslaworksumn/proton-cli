use std::path::PathBuf;
use std::cmp;

use error::Error;
use project_types::SequenceSection;
use utils;


#[derive(Clone, Debug, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub struct Sequence {
    pub name: String,
    pub directory_name: String,
    pub music_file_name: String,
    pub music_duration_sec: u32,
    pub frame_duration_ms: u32,
    pub num_sections: u32,
}

impl Sequence {
    /// Creates a new Sequence, allowing default values
    /// of 50ms for frame_duration_ms and 1 for num_sections
    /// Also initializes section file(s)
    pub fn new(name: &str,
        seq_directory_name: &str,
        music_file_name: &str,
        music_duration_sec: u32,
        frame_duration_ms: Option<u32>,
        num_sections: Option<u32>
    ) -> Result<Sequence, Error> {
        // Defaults
        // TODO: put constraint (>25ms)
        let frame_dur_ms = frame_duration_ms.unwrap_or(50);
        // TODO: put constraint (!= 0)
        let num_sects = num_sections.unwrap_or(1);

        // Create sequence
        let mut sequence = Sequence {
            name: name.to_string(),
            directory_name: seq_directory_name.to_string(),
            music_file_name: music_file_name.to_string(),
            music_duration_sec: music_duration_sec,
            frame_duration_ms: frame_dur_ms,
            num_sections: 0 // Updated in resection
        };

        // Section sequence
        try!(sequence.resection(num_sects));

        Ok(sequence)
    }

    /// Reads in a sequence section from its file
    pub fn get_section(&self, section: u32) -> Result<SequenceSection, Error> {
        if section > 0 && section <= self.num_sections {
            let section_path = self.get_section_path(section);
            utils::read_sequence_section(&section_path)
        } else {
            Err(Error::InvalidSequenceSection(section))
        }
    }

    /// Reads in all sequence sections from their files into a vec
    fn get_all_sections(&self) -> Result<Vec<SequenceSection>, Error> {
        let mut sections = vec![];
        for i in 0..self.num_sections {
            let section = try!(self.get_section(i + 1));
            sections.push(section);
        }
        Ok(sections)
    }

    /// Generate and return a sequence section with sane defaults
    /// Also writes it to file, so it can be read later
    fn create_default_section(
        &mut self,
        num_channels: u32,
        num_frames: u32
    ) -> Result<SequenceSection, Error> {        
        let section_path = self.get_section_path(1);
        let section = SequenceSection {
            seq_name: self.name.to_string(),
            index: 1,
            path: section_path.clone(),
            num_frames: num_frames,
            editor: None,
            data: vec![vec![0; num_frames as usize]; num_channels as usize],
        };
        let _ = try!(section.write_to_file());
        Ok(section)
    }

    /// Resection a sequence
    pub fn resection(&mut self, num_sections: u32) -> Result<(), Error> {
        // No need to resection if same number of sections
        if self.num_sections == num_sections {
            return Ok(());
        }

        let num_channels: u32 = 1; // TODO change when add layout
        let music_duration_ms = self.music_duration_sec as f32 * 1000_f32;
        let num_frames_f32: f32 = music_duration_ms as f32 / self.frame_duration_ms as f32;
        let num_frames = num_frames_f32.ceil() as u32;

        // Turn into one section if not already, then split up
        let mut sections = try!(self.get_all_sections());
        // If no sections (first run?), initialize with default
        if sections.len() < 1 {
            let default_section = try!(self.create_default_section(num_channels, num_frames));
            sections = vec![default_section];
        }
        // Function to combine data vectors in the fold
        fn combine_data(mut accumulated: Vec<Vec<u8>>, sec: &SequenceSection) -> Vec<Vec<u8>> {
            let mut sec_data = sec.data.clone();
            let min_num_ch = cmp::min(accumulated.len(), sec_data.len());
            for channel in 0..min_num_ch {
                accumulated[channel].append(&mut sec_data[channel]);
            }
            accumulated
        }
        // Fold together all data vectors
        let all_data = sections.iter().fold(vec![vec![]; num_channels as usize], combine_data);
        if num_sections == 1 {
            let _ = try!(self.create_default_section(num_channels, num_frames));
            self.num_sections = num_sections;
            Ok(())
        } else {
            Err(Error::TodoErr)
        }
    }

    /// Get the path to this specific section, starting with the sequence directory
    /// E.g. sequence/sequence_section1.json
    /// Assumes the current directory is the project directory
    /// Returns as string
    fn get_section_path(
        &self,
        index: u32
    ) -> String {

        let mut filename = String::new();
        filename.push_str(&self.name);
        filename.push_str("_section");
        filename.push_str(&index.to_string());

        let mut section_path = PathBuf::from(&self.directory_name);
        section_path.push(&filename);
        
        section_path.to_str().expect("Path is invalid unicode").to_owned()
    }

}

