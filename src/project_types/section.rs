use error::Error;

#[derive(Debug)]
pub struct Section {
    pub secid: u32,
    pub t_start: u32,
    pub t_end: u32,
    pub seqid: u32,
    pub fixtures: Vec<u32>,
}

impl Section {

    /// Creates a new section
    #[allow(unused_variables)]
    pub fn new(
        uid: u32,
        seqid: u32,
        t_start: u32,
        t_end: u32,
        fixtures: Vec<u32>
    ) -> Result<Section, Error> {
        // Make sure user has permission to add section
        // Make sure seqid is a valid sequence
        // Make sure t_start is not before the end of the sequence
        // Make sure t_end is >= t_start
        // Make sure fixtures contains only valid fixtures (and is not empty)

        Err(Error::TodoErr)
    }

}