use std::path::{Path, PathBuf};
use std::cmp;

use git2::Signature;

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
        let frame_dur_ms = frame_duration_ms.unwrap_or(50);
        if frame_dur_ms < 25 {
            return Err(Error::InvalidFrameDuration(frame_dur_ms));
        }
        let num_sects = num_sections.unwrap_or(1);
        if num_sects < 1 {
            return Err(Error::InvalidNumSequenceSections(num_sects));
        }

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
        if self.section_in_range(section) {
            let section_path = self.get_section_path(section);
            utils::read_sequence_section(&section_path)
        } else {
            Err(Error::InvalidSequenceSection(section))
        }
    }

    /// Checks to see if the given section is valid/in range
    pub fn section_in_range(&self, section: u32) -> bool {
        section < self.num_sections
    }

    /// Reads in all sequence sections from their files into a vec
    fn get_all_sections(&self) -> Result<Vec<SequenceSection>, Error> {
        (0..self.num_sections).map(|i| self.get_section(i)).collect()
    }

    /// Generate and return a sequence section with sane defaults
    /// Also writes it to file, so it can be read later
    fn create_default_section(
        &self,
        num_channels: u32,
        num_frames: u32
    ) -> Result<SequenceSection, Error> {
        self.create_section(
            1,
            num_frames,
            vec![vec![0; num_frames as usize]; num_channels as usize]
        )
    }

    /// Creates a sequence section, writes it to a file, and returns it
    fn create_section(
        &self,
        index: u32,
        num_frames: u32,
        data: Vec<Vec<u8>>
    ) -> Result<SequenceSection, Error> {
        let section_path = self.get_section_path(index);
        let section = SequenceSection {
            seq_name: self.name.to_string(),
            index: index,
            path: section_path.clone(),
            num_frames: num_frames,
            data: data
        };
        let _ = try!(section.write_to_file());
        Ok(section)
    }

    /// Sets the data for a sequence section
    /// Writes the changes and commits
    pub fn set_section_data(&self, index: u32, data: Vec<Vec<u8>>) -> Result<(), Error> {
        let mut seq_sec = try!(self.get_section(index));
        seq_sec.data = data;
        let _ = try!(seq_sec.write_to_file());

        let signature = Signature::now("Proton Lights", "proton@teslaworks.net").unwrap();
        let msg = format!("Setting section data for sequence '{}', section {}", self.name, index);
        let repo_path: Option<&Path> = None;

        utils::commit_all(repo_path, &signature, &msg)
            .map(|_| ())
    }

    /// Resection a sequence
    pub fn resection(&mut self, num_sections: u32) -> Result<(), Error> {
        // No need to resection if same number of sections
        if self.num_sections == num_sections {
            return Ok(());
        }
        // Cannot resection into 0 sections
        if num_sections == 0 {
            return Err(Error::InvalidSequenceSection(num_sections));
        }

        let num_channels: u32 = 3; // TODO change when add layout
        let music_duration_ms = self.music_duration_sec as f32 * 1000_f32;
        let num_frames_f32: f32 = music_duration_ms as f32 / self.frame_duration_ms as f32;
        let num_frames = num_frames_f32.ceil() as u32;
        let num_frames_per_section_f32 = num_frames_f32 / num_sections as f32;
        let num_frames_per_section = num_frames_per_section_f32.ceil() as u32;

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
        let all_data = sections.iter()
            .fold(vec![vec![]; num_channels as usize], combine_data);
        // Break single chunk into sections, one channel at a time
        let mut sectioned_data = vec![vec![vec![]; num_channels as usize]; num_sections as usize];
        for (channel_idx, channel_data) in all_data.iter().enumerate() {
            let mut chunked_data = channel_data.chunks(num_frames_per_section as usize);
            for section_idx in 0..num_sections {
                let new_data = chunked_data.next()
                    .expect("Miscalculation when chunking sequence section data");
                sectioned_data[section_idx as usize][channel_idx as usize] = new_data.to_vec();
            }
        }

        // Create SequenceSections out of the chunks
        for (sec_idx, sec_data) in sectioned_data.iter().enumerate() {
            // This is safe, since there is always at least one channel
            let sec_frames = sec_data[0].len();

            let _ = self.create_section(
                sec_idx as u32,
                sec_frames as u32,
                sec_data.to_owned());
        }

        Ok(self.num_sections = num_sections)
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

