use postgres::Connection;

use project_types::Fixture;
use error::Error;
use dao::{FixtureDao, FixtureDaoPostgres};

impl FixtureDao for FixtureDaoPostgres {
    fn get_fixture(&self, fixid: u32) -> Result<Fixture, Error> {
        Err(Error::TodoErr)
    }

    fn get_num_channels(&self, fixid: u32) -> Result<u32, Error> {
        let query = "SELECT array_length(channels,1) FROM fixtures WHERE fixid = $1";
        let results = try!(
            self.conn.query(query, &[&(fixid as i32)])
            .map_err(Error::Postgres));
        match results.len() {
            0 => Err(Error::FixtureNotFound(fixid)),
            1 => {
                let row = results.get(0);
                let num_channels: i32 = row.get(0);
                Ok(num_channels as u32)
            },
            x => Err(Error::InvalidNumResults(x)),
        }
    }
}
