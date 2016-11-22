use dao::{DataDao, DataDaoPostgres};
use error::Error;


impl DataDao for DataDaoPostgres {

    fn new_data(
        &self,
        seqid: u32,
        chan_ids: Vec<u32>,
        default_data: Vec<u16>
    ) -> Result<(), Error> {
        let statement = "INSERT INTO channel_data (chanid,seqid,data) VALUES ($1,$2,$3)";
        let insert_stmt = try!(self.conn.prepare(statement).map_err(Error::Postgres));
        let default_data_i32 = default_data.iter()
            .map(|frame| *frame as i32)
            .collect::<Vec<i32>>();
        for chanid in chan_ids {
            let _ = try!(
                insert_stmt.execute(
                    &[
                        &(chanid as i32),
                        &(seqid as i32),
                        &default_data_i32
                    ])
                .map_err(Error::Postgres));
        }
        Ok(())
    }
}
