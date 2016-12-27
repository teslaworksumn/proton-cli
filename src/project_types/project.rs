use error::Error;


/// Structure to represent a Proton Project.
#[derive(Clone, Debug, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub struct Project {
    pub name: String,
    pub playlist: Vec<u32>,
    pub layout_id: u32,
}

impl Project {

    /// Checks to see that the given project name is valid (alphanumerics and spaces)
    pub fn validate_name(name: &str) -> bool {
        name.chars().all(|c| c.is_alphanumeric() || c == ' ')
    }

    /// Inserts a sequence in the project's playlist at the given offset
    pub fn insert_sequence(&self, seqid: u32, offset: u32) -> Result<Project, Error> {

        // Check if offset is out of bounds
        if offset > self.playlist.len() as u32 {
            return Err(Error::OffsetOutOfBounds(offset, self.playlist.len() as u32));
        }

        // Check if seqid exists?? Assume it is checked earlier for now
        let mut new_project = self.clone();
        new_project.playlist.insert(offset as usize, seqid);

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
