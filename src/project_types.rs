
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use rustc_serialize::json;

use Error;

/// Structure to represent a Proton Project.
/// This is what will be written to a Protonfile at the project root.
#[derive(Clone, Debug, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub struct Project {
    pub name: String,
    pub users: Vec<User>,
    pub sequences: Vec<Sequence>,
}

#[derive(Clone, Debug, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub struct User {
    pub name: String,
    pub public_key: String,
}

#[derive(Clone, Debug, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub struct SequenceSection {
    pub seq_name: String,
    pub index: u32,
    pub num_frames: u32,
    pub editor: Option<User>,
    pub data: Vec<Vec<u8>>, // Row is channel, column is frame    
}

#[derive(Clone, Debug, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub struct Sequence {
    pub name: String,
    pub directory_name: String,
    pub music_file_name: String,
    pub music_duration_sec: u32,
    pub frame_duration_ms: u32,
    pub num_sections: u32,
}

impl Project {
    /// Creates an empty project
    pub fn empty() -> Project {
        Project {
            name: "New Project".to_owned(),
            users: Vec::new(),
            sequences: Vec::new(),
        }
    }

    /// Finds a user with the given public key
    /// Returns the user if found, else None
    fn find_user_by_public_key(&self, pub_key: &str) -> Option<&User> {
        for u in &self.users {
            if u.public_key == pub_key {
                return Some(u);
            }
        }
        None::<&User>
    }

    /// Finds a user with the given name
    /// Returns the user if found, else None
    fn find_user_by_name(&self, name: &str) -> Option<&User> {
        for u in &self.users {
            if u.name == name {
                return Some(u);
            }
        }
        None::<&User>
    }

    /// Finds a user in the users vector
    /// Returns true if found, else false
    pub fn user_exists(&self, user: &User) -> bool {
        for u in &self.users {
            if user == u {
                return true;
            }
        }
        return false;
    }

    /// Adds a user to the project
    /// Returns a new project with the user added
    pub fn add_user(&self, name: &str, pub_key: &str) -> Result<Project, Error> {
        
        let user = User {
            name: name.to_string(),
            public_key: pub_key.to_string(),
        };

        if self.find_user_by_name(name).is_some() ||
           self.find_user_by_public_key(pub_key).is_some() {
           
            Err(self.duplicate_user(pub_key, name))
        } else {
            let mut new_project = self.clone();
            new_project.users.push(user);
            Ok(new_project)
        }
    }

    fn duplicate_user(&self, pub_key: &str, name: &str) -> Error {
        Error::DuplicateUser(pub_key.to_string(), name.to_string())
    }

    /// Adds a sequence to the project
    /// Returns a new project with the sequence added
    pub fn add_sequence(
        &self,
        name: &str,
        directory_name: &str,
        music_file_name: &str,
        music_duration_sec: u32,
    ) -> Result<Project, Error> {
    
        let sequence = try!(Sequence::new(
            name,
            directory_name,
            music_file_name,
            music_duration_sec,
            None,
            None,
        ));

        // Check if duplicate
        let mut exists = false;
        for s in &self.sequences {
            if s.name == name
            || s.directory_name == directory_name {
                exists = true;
                break;
            }
        }

        if exists {
            Err(duplicate_sequence(name))
        } else {
            let mut new_project = self.clone();
            new_project.sequences.push(sequence);
            Ok(new_project)
        }
    }
}

fn duplicate_sequence(name: &str) -> Error {
    Error::DuplicateSequence(name.to_string())
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
        let num_sects = num_sections.unwrap_or(1);

        // Create sequence
        let sequence = Sequence {
            name: name.to_string(),
            directory_name: seq_directory_name.to_string(),
            music_file_name: music_file_name.to_string(),
            music_duration_sec: music_duration_sec,
            frame_duration_ms: frame_dur_ms,
            num_sections: num_sects,
        };

        // Section sequence
        try!(sequence.resection());

        Ok(sequence)
    }

    /// Resection a sequence
    pub fn resection(&self) -> Result<(), Error> {
        if self.num_sections == 1 {
            let num_frames_f32: f32 = (self.music_duration_sec as f32 * 1000_f32) / self.frame_duration_ms as f32;
            let num_frames = num_frames_f32.ceil() as u32;
            let num_channels = 1; // TODO: change when add layout
            let sequence_section = SequenceSection {
                seq_name: self.name.to_string(),
                index: 1,
                num_frames: num_frames,
                editor: None,
                data: vec![vec![0; num_frames as usize]; num_channels],
            };
            sequence_section.write_to_file(&self.directory_name)
        } else {
            Err(Error::TodoErr)
        }
    }

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

