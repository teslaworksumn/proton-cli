use project_types::Permission;
use dao::{PermissionDao, PermissionDaoPostgres};
use error::Error;


impl PermissionDao for PermissionDaoPostgres {

    fn get_all_permissions(&self, uid: u32) -> Result<Vec<Permission>, Error> {
        Err(Error::TodoErr)
    }

    fn get_permission(&self, permid: u32) -> Result<Permission, Error> {
        Err(Error::TodoErr)
    }
}
