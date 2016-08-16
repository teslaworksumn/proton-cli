use dao::LayoutDao;
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

    /// Adds a sequence to the project's playlist
    pub fn add_sequence(&self, seqid: u32) -> Result<Project, Error> {

        // Check if seqid exists?? Assume it is checked earlier for now
        let mut new_project = self.clone();
        new_project.playlist.push(seqid);
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
