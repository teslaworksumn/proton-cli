
use dao::LayoutDao;
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

    pub fn empty<L: LayoutDao>(
        layout_dao: L,
        name: &str,
        layout_id: Option<u32>
    ) -> Result<Project, Error> {

        // Check that layout_id is valid if given
        let layout = match layout_id {
            Some(lid) => try!(layout_dao.get_layout(lid)),
            None => try!(layout_dao.get_default_layout())
        };

        // Create new Rroject
        Ok(Project {
            name: name.to_owned(),
            playlist: vec![],
            layout_id: layout.layout_id
        })
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
