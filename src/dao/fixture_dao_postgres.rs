use project_types::Fixture;
use error::Error;
use dao::{FixtureDao, FixtureDaoPostgres};

impl FixtureDao for FixtureDaoPostgres {

    fn new_fixture(
        &self, 
        name: &str,
        location: (i32, i32, i32),
        rotation: (i32, i32, i32),
        channels: Vec<u32>
    ) -> Result<Fixture, Error> {
        let statement = "INSERT INTO fixtures (name,location_x,location_y,location_z,\
            rotation_a,rotation_b,rotation_c,channels) VALUES ($1,$2,$3,$4,$5,$6,$7,$8)";
        let channels_i32 = channels.iter()
            .map(|channel| *channel as i32)
            .collect::<Vec<i32>>();
        let _ = try!(
            self.conn.execute(
                statement,
                &[
                    &name.to_owned(),
                    &location.0,
                    &location.1,
                    &location.2,
                    &rotation.0,
                    &rotation.1,
                    &rotation.2,
                    &channels_i32
                ])
            .map_err(Error::Postgres));

        let fixture = try!(self.get_last_fixture(name));
        Ok(fixture)
    }

    fn get_last_fixture(&self, name: &str) -> Result<Fixture, Error> {
        let query = "SELECT fixid,location_x,location_y,location_z,rotation_a,rotation_b,\
            rotation_c,channels FROM fixtures WHERE name = $1 ORDER BY fixid DESC";
        let results = try!(
            self.conn.query(query, &[&name.to_owned()])
            .map_err(Error::Postgres));
        
        if results.len() == 0 {
            return Err(Error::FixtureNotFound(0));
        }

        // First row has largest fixid (most recently added with name)
        let row = results.get(0);
        let fixid: i32 = row.get(0);
        let location_x: i32 = row.get(1);
        let location_y: i32 = row.get(2);
        let location_z: i32 = row.get(3);
        let rotation_a: i32 = row.get(4);
        let rotation_b: i32 = row.get(5);
        let rotation_c: i32 = row.get(6);
        let channels_i32: Vec<i32> = row.get(7);
        let channels = channels_i32.iter()
            .map(|channel| *channel as u32)
            .collect::<Vec<u32>>();
        Ok(Fixture {
            fixid: fixid as u32,
            name: name.to_owned(),
            location: (location_x, location_y, location_z),
            rotation: (rotation_a, rotation_b, rotation_c),
            channels: channels
        })
    }

    #[allow(unused_variables)]
    fn get_fixture(&self, fixid: u32) -> Result<Fixture, Error> {
        Err(Error::TodoErr)
    }

    fn fixture_exists(&self, fixid: u32) -> Result<bool, Error> {
        let query = "SELECT fixid FROM fixtures WHERE fixid = $1";
        let results = try!(
            self.conn.query(query, &[&(fixid as i32)])
            .map_err(Error::Postgres));
        Ok(results.len() > 0)
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
