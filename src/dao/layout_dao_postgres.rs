use project_types::Layout;
use error::Error;
use dao::{LayoutDao, LayoutDaoPostgres};


impl LayoutDao for LayoutDaoPostgres {
    fn get_layout(&self, layoutid: u32) -> Result<Layout, Error> {
        Err(Error::TodoErr)
    }
}
