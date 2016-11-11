use project_types::Layout;
use error::Error;
use dao::{LayoutDao, LayoutDaoPostgres};


impl LayoutDao for LayoutDaoPostgres {
    
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
        let query = "SELECT * FROM layouts WHERE layoutid = $1";
        let lid = layout_id as i32;
        let results = try!(
            self.conn.query(query, &[&lid])
            .map_err(Error::Postgres));
        match results.len() {
            0 => Err(Error::LayoutNotFound(layout_id)),
            1 => {
                Err(Error::TodoErr)
            },
            x => Err(Error::InvalidNumResults(x)),
        }
    }
}
