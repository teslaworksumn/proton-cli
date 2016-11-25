use dao::{DataDao, DataDaoPostgres};
use error::Error;


impl DataDao for DataDaoPostgres {

    fn new_data_default(
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

    fn new_data<'a>(
        &'a self,
        seqid: u32,
        chanid: u32,
        new_data: &'a Vec<u16>
    ) -> Result<(), Error> {
        let statement = "INSERT INTO channel_data (chanid,seqid,data) VALUES ($1,$2,$3)";
        let new_data_i32 = new_data.iter()
            .map(|frame| *frame as i32)
            .collect::<Vec<i32>>();
        let _ = try!(
            self.conn.execute(
                statement,
                &[
                    &(chanid as i32),
                    &(seqid as i32),
                    &new_data_i32
                ])
            .map_err(Error::Postgres));
        Ok(())
    }

    fn get_data(&self, seqid: u32, chanid: u32) -> Result<Vec<u16>, Error> {
        let query = "SELECT data FROM channel_data WHERE seqid = $1 AND chanid = $2";
        let results = try!(
            self.conn.query(query, &[&(seqid as i32), &(chanid as i32)])
            .map_err(Error::Postgres));
        match results.len() {
            0 => Err(Error::ChannelDataNotFound(seqid, chanid)),
            1 => {
                let row = results.get(0);
                let data: Vec<i32> = row.get(0);
                let data_u16 = data.iter()
                    .map(|frame_val| *frame_val as u16)
                    .collect::<Vec<u16>>();
                Ok(data_u16)
            },
            x => Err(Error::InvalidNumResults(x)),
        }
    }

    fn update_data<'a>(&'a self, seqid: u32, chanid: u32, new_data: &'a Vec<u16>) -> Result<(), Error> {
        let statement = "UPDATE channel_data SET data = $1 WHERE seqid = $2 AND chanid = $3";
        let new_data_i32 = new_data.iter()
            .map(|val_u16| *val_u16 as i32)
            .collect::<Vec<i32>>();
        let _ = try!(
            self.conn.execute(
                statement,
                &[
                    &new_data_i32,
                    &(seqid as i32),
                    &(chanid as i32)
                ])
            .map_err(Error::Postgres));
        Ok(())
    }
}
