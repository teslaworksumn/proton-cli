use project_types::Layout;
use error::Error;
use dao::{LayoutDao, LayoutDaoPostgres};


impl LayoutDao for LayoutDaoPostgres {

    fn new_layout(&self, name: &str, fixtures: Vec<u32>) -> Result<Layout, Error> {
        let statement = "INSERT INTO layouts (name,fixtures) VALUES ($1,$2)";
        let fixtures_i32 = fixtures.iter()
            .map(|fixture| *fixture as i32)
            .collect::<Vec<i32>>();
        let _ = try!(
            self.conn.execute(
                statement,
                &[
                    &name.to_owned(),
                    &fixtures_i32
                ])
            .map_err(Error::Postgres));

        let layout = try!(self.get_last_layout(name));
        Ok(layout)
    }

    fn patch_channel(
        &self,
        layoutid: u32,
        channel_internal: u32,
        channel_dmx: u32
    ) -> Result<u64, Error> {

        let statement = "UPDATE channels SET channel_dmx=$1 \
        WHERE chanid = get_internal_chan_id($2, $3)";
        let rows_altered = try!(
            self.conn.execute(statement, &[
                &(channel_dmx as i32),
                &(layoutid as i32),
                &(channel_internal as i32)
            ])
            .map_err(Error::Postgres));

        Ok(rows_altered)
    }

    fn get_last_layout(&self, name: &str) -> Result<Layout, Error> {
        let query = "SELECT layoutid, fixtures FROM layouts WHERE name = $1 ORDER BY layoutid DESC";
        let results = try!(
            self.conn.query(query, &[&name.to_owned()])
            .map_err(Error::Postgres));
        if results.len() == 0 {
            return Err(Error::LayoutNotFound(0));
        }

        let row = results.get(0);
        let layout_id: i32 = row.get(0);
        let fixtures_i32: Vec<i32> = row.get(1);
        let fixtures = fixtures_i32.iter()
            .map(|fixture| *fixture as u32)
            .collect::<Vec<u32>>();
        Ok(Layout {
            layout_id: layout_id as u32,
            name: name.to_owned(),
            fixtures: fixtures
        })
    }
    
    fn get_default_layout(&self) -> Result<Layout, Error> {
        let query = "SELECT layoutid, name, fixtures FROM layouts WHERE name = 'default'";
        let results = try!(
            self.conn.query(query, &[])
            .map_err(Error::Postgres));
        match results.len() {
            0 => Err(Error::LayoutNotFound(0)),
            1 => {
                let row = results.get(0);
                let layoutid: i32 = row.get(0);
                let name: String = row.get(1);
                let fixtures: Vec<i32> = row.get(2);
                let fixtures_u32 = fixtures.iter().map(|x| x.to_owned() as u32).collect();
                Ok(Layout {
                    layout_id: layoutid as u32,
                    name: name,
                    fixtures: fixtures_u32
                })
            },
            x => Err(Error::InvalidNumResults(x)),
        }
    }

    fn get_layout(&self, layout_id: u32) -> Result<Layout, Error> {
        let query = "SELECT name, fixtures FROM layouts WHERE layoutid = $1";
        let lid = layout_id as i32;
        let results = try!(
            self.conn.query(query, &[&lid])
            .map_err(Error::Postgres));
        match results.len() {
            0 => Err(Error::LayoutNotFound(layout_id)),
            1 => {
                let row = results.get(0);
                let name: String = row.get(0);
                let fixtures: Vec<i32> = row.get(1);
                let fixtures_u32 = fixtures.iter().map(|x| x.to_owned() as u32).collect();
                Ok(Layout {
                    layout_id: layout_id,
                    name: name,
                    fixtures: fixtures_u32
                })
            },
            x => Err(Error::InvalidNumResults(x)),
        }
    }

    fn layout_exists(&self, layout_id: u32) -> Result<bool, Error> {
        let query = "SELECT layoutid FROM layouts WHERE layoutid = $1";
        let results = try!(
            self.conn.query(query, &[&(layout_id as i32)])
            .map_err(Error::Postgres));
        Ok(results.len() > 0)
    }
}
