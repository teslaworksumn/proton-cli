use project_types::Section;
use dao::{SectionDao, SectionDaoPostgres};
use error::Error;


impl SectionDao for SectionDaoPostgres {
    fn get_section(&self, secid: u32) -> Result<Section, Error> {
        Err(Error::TodoErr)
    }
}
