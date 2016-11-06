use project_types::Sequence;
use dao::SequenceDao;
use error::Error;

pub struct SequenceDaoPostgres{}

impl SequenceDao for SequenceDaoPostgres {
    fn get_sequence(&self, seqid: u32) -> Result<Sequence, Error> {
        Err(Error::TodoErr)
    }
}