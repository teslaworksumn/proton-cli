use postgres::Connection;

use project_types::Fixture;
use error::Error;
use dao::{FixtureDao, FixtureDaoPostgres};

impl FixtureDao for FixtureDaoPostgres {
    fn get_fixture(&self, fixid: u32) -> Result<Fixture, Error> {
        Err(Error::TodoErr)
    }
}
