
use project_types::{Sequence, User, Permission};
use error::Error;


/// Structure to represent a Proton Project.
/// This is what will be written to a Protonfile at the project root.
#[derive(Clone, Debug, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub struct Project {
    pub name: String,
    pub playlist: Vec<u32>,
    pub layout_id: u32,
}

impl Project {

    pub fn empty(name: &str, layout_id: Option<u32>) -> Result<Project, Error> {
        // Check that layout_id is valid if given
        // Validate name characters??
        Err(Error::TodoErr)
    }

    /// Finds a sequence by its id
    /// Returns the sequence if found, else None
    pub fn find_sequence_by_seqid(&self, seqid: u32) -> Option<&Sequence> {
        None
    }

    /// Finds a user with the given id
    /// Returns the user if found, else None
    fn find_user_by_uid(&self, uid: u32) -> Option<&User> {
        None
    }

    /// Adds a user to the users table
    pub fn add_user(&self, name: &str, pub_key: &str) -> Result<(), Error> {
        let user = try!(User::new(name, pub_key));
        // Insert into users table
        Err(Error::TodoErr)
    }

    /// Removes a user from the users table
    pub fn remove_user(&self, uid: u32) -> Result<Project, Error> {
        // Update table, drop row where uid = uid
        // Return different error if UserNotFound
        Err(Error::TodoErr)
    }

    /// Adds a sequence to the project
    pub fn add_sequence(
        &self,
        uid: u32,
        name: &str,
        music_file_name: &str,
        music_duration_sec: u32,
        frame_duration_ms: Option<u32>
    ) -> Result<Project, Error> {

        let sequence = try!(Sequence::new(
            uid,
            name,
            music_file_name,
            music_duration_sec,
            frame_duration_ms,
            self.layout_id
        ));

        // Check if duplicate name (part of error returned by Sequence::new)

        // Add sequence to playlist
        let mut new_project = self.clone();
        new_project.playlist.push(sequence.seqid);
        Ok(new_project)
    }

    // Removes sequence from project playlist if it exists, error if not found
    pub fn remove_sequence(&self, seqid: u32) -> Result<Project, Error> {
        let mut new_project = self.clone();
        for (i, seq) in self.playlist.iter().enumerate() {
            if *seq == seqid {
                new_project.playlist.remove(i);
                return Ok(new_project);
            }
        }
        Err(Error::SequenceNotFound(seqid))
    }
}
