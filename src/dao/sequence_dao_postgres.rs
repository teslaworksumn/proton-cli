use project_types::Sequence;
use dao::{SequenceDao, SequenceDaoPostgres};
use error::Error;


impl SequenceDao for SequenceDaoPostgres {
    fn get_sequence(&self, seqid: u32) -> Result<Sequence, Error> {
        Err(Error::TodoErr)
    }
}