use dao::FixtureDao;
use error::Error;

/// Contains a layout (a logical collection of fixtures)
#[derive(Debug)]
pub struct Layout {
    pub layout_id: u32,
    pub name: String,
    pub fixtures: Vec<u32>,
}

impl Layout {
    // Gets the total number of channels contained within this layout's fixtures
    pub fn get_num_channels<FD: FixtureDao>(&self, fixture_dao: &FD) -> Result<u32, Error> {
        let mut count = 0;
        for fixture in &self.fixtures {
            let fix_chan_count = try!(fixture_dao.get_num_channels(*fixture));
            count += fix_chan_count;
        }
        Ok(count)
    }
}
