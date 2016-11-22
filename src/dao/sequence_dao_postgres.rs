use rustc_serialize::json;

use dao::{SequenceDao, SequenceDaoPostgres};
use error::Error;
use project_types::Sequence;
use utils;


impl SequenceDao for SequenceDaoPostgres {
    fn get_sequence(&self, seqid: u32) -> Result<Sequence, Error> {
        let query = "SELECT name,music_file_name,music_dur_sec,frame_dur_ms,num_frames,layout_id,\
        data FROM sequences WHERE seqid = $1";
        let results = try!(
            self.conn.query(query, &[&(seqid as i32)])
            .map_err(Error::Postgres));
        match results.len() {
            0 => Err(Error::SequenceNotFound(seqid)),
            1 => {
                let row = results.get(0);
                let name: String = row.get(0);
                let music_file_name: String = row.get(1);
                let music_dur_sec: i32 = row.get(2);
                let frame_dur_ms: i32 = row.get(3);
                let num_frames: i32 = row.get(4);
                let layout_id: i32 = row.get(5);
                let data_json: json::Json = row.get(6);
                let data_u16 = utils::sequence_json_to_vec(data_json);
                Ok(Sequence {
                    seqid: seqid,
                    name: name,
                    music_file_name: music_file_name,
                    music_duration_sec: music_dur_sec as u32,
                    frame_duration_ms: frame_dur_ms as u32,
                    num_frames: num_frames as u32,
                    layout_id: layout_id as u32,
                    data: data_u16,
                })
            },
            x => Err(Error::InvalidNumResults(x)),
        }
    }

    fn get_last_sequence(&self, name: &str) -> Result<Sequence, Error> {
        let query = "SELECT seqid,music_file_name,music_dur_sec,frame_dur_ms,num_frames,layout_id,\
        data FROM sequences WHERE name = $1 ORDER BY seqid DESC";
        let results = try!(
            self.conn.query(query, &[&name.to_owned()])
            .map_err(Error::Postgres));
        if results.len() == 0 {
            return Err(Error::SequenceNotFound(0));
        }

        let row = results.get(0);
        let seqid: i32 = row.get(0);
        let music_file_name: String = row.get(1);
        let music_dur_sec: i32 = row.get(2);
        let frame_dur_ms: i32 = row.get(3);
        let num_frames: i32 = row.get(4);
        let layout_id: i32 = row.get(5);
        let data_json: json::Json = row.get(6);
        let data_u16 = utils::sequence_json_to_vec(data_json);
        Ok(Sequence {
            seqid: seqid as u32,
            name: name.to_owned(),
            music_file_name: music_file_name,
            music_duration_sec: music_dur_sec as u32,
            frame_duration_ms: frame_dur_ms as u32,
            num_frames: num_frames as u32,
            layout_id: layout_id as u32,
            data: data_u16,
        })
    }

    fn new_sequence(&self, sequence: &Sequence) -> Result<Sequence, Error> {
        let statement = "INSERT INTO sequences (name,music_file_name,music_dur_sec,\
            frame_dur_ms,num_frames,layout_id,data) VALUES ($1,$2,$3,$4,$5,$6,$7)";
        let music_dur = sequence.music_duration_sec as i32;
        let frame_dur = sequence.frame_duration_ms as i32;
        let num_frames = sequence.num_frames as i32;
        let layout_id = sequence.layout_id as i32;
        let data = try!(sequence.data_as_json());
        let _ = try!(
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
        let sequence = try!(self.get_last_sequence(&sequence.name));
        Ok(sequence)
    }
}