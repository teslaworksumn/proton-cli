use rustc_serialize::json;

use project_types::Sequence;
use dao::{SequenceDao, SequenceDaoPostgres};
use error::Error;


impl SequenceDao for SequenceDaoPostgres {
    fn get_sequence(&self, seqid: u32) -> Result<Sequence, Error> {
        Err(Error::TodoErr)
    }

    fn new_sequence(&self, sequence: &Sequence) -> Result<(), Error> {
        let statement = "INSERT INTO sequences (name,music_file_name,music_dur_sec,\
            frame_dur_ms,num_frames,layout_id,data) VALUES ($1,$2,$3,$4,$5,$6,$7)";
        let music_dur = sequence.music_duration_sec as i32;
        let frame_dur = sequence.frame_duration_ms as i32;
        let num_frames = sequence.num_frames as i32;
        let layout_id = sequence.layout_id as i32;
        let data = try!(sequence.data_as_json());
        let rows_modified = try!(
            self.conn.execute(
                statement,
                &[
                    &sequence.name.to_owned(),
                    &sequence.music_file_name.to_owned(),
                    &music_dur,
                    &frame_dur,
                    &num_frames,
                    &layout_id,
                    &data
                ])
            .map_err(Error::Postgres));
        Ok(())
    }
}