use std::path::PathBuf;

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
    pub section_paths: Vec<String>,
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
        let sequence = Sequence {
            name: name.to_string(),
            directory_name: seq_directory_name.to_string(),
            music_file_name: music_file_name.to_string(),
            music_duration_sec: music_duration_sec,
            frame_duration_ms: frame_dur_ms,
            num_sections: num_sects,
            section_paths: vec![]  // Initialized in resection
        };

        // Section sequence
        let resectioned_sequence = try!(sequence.resection(num_sects));

        Ok(resectioned_sequence)
    }

    pub fn get_section(&self, section: u32) -> Result<SequenceSection, Error> {
        if section > 0 && section <= self.num_sections {
            // Sections are 1-indexed
            let section_path = &self.section_paths[section as usize - 1];
            utils::read_sequence_section(&section_path)
        } else {
            Err(Error::InvalidSequenceSection)
        }
    }

    /// Resection a sequence
    pub fn resection(&self, num_sections: u32) -> Result<Sequence, Error> {
        let mut sequence = self.clone();
        sequence.num_sections = num_sections;
        if num_sections == 1 {
            let section_path = sequence.get_section_path(
                &sequence.directory_name,
                &sequence.name,
                1);
            sequence.section_paths.push(section_path.clone());
            let music_duration_ms = sequence.music_duration_sec as f32 * 1000_f32;
            let num_frames_f32: f32 = music_duration_ms as f32 / sequence.frame_duration_ms as f32;
            let num_frames = num_frames_f32.ceil() as u32;
            let num_channels = 1; // TODO: change when add layout
            let sequence_section = SequenceSection {
                seq_name: sequence.name.to_string(),
                index: 1,
                path: section_path,
                num_frames: num_frames,
                editor: None,
                data: vec![vec![0; num_frames as usize]; num_channels],
            };
            let _ = try!(sequence_section.write_to_file());
            Ok(sequence)
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
        directory_name: &str,
        seq_name: &str,
        index: u32
    ) -> String {

        let mut filename = String::new();
        filename.push_str(&seq_name);
        filename.push_str("_section");
        filename.push_str(&index.to_string());

        let mut section_path = PathBuf::from(&directory_name);
        section_path.push(&filename);
        
        section_path.to_str().expect("Path is invalid unicode").to_owned()
    }

}

