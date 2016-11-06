use project_types::Section;
use dao::SectionDao;
use error::Error;

pub struct SectionDaoPostgres{}

impl SectionDao for SectionDaoPostgres {
    fn get_section(&self, secid: u32) -> Result<Section, Error> {
        Err(Error::TodoErr)
    }
}

