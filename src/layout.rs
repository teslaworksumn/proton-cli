use std::path::Path;

use dao::{LayoutDao, PermissionDao, UserDao};
use error::Error;
use project_types::{Layout, PermissionEnum};
use utils;

pub fn new_layout<P: AsRef<Path>, LD: LayoutDao, PD: PermissionDao, UD: UserDao>(
    layout_dao: &LD,
    perm_dao: &PD,
    user_dao: &UD,
    admin_key_path: P,
    layout_path: P,
) -> Result<u32, Error> {

    // Check that the admin has sufficient privileges
    let valid_permissions = vec![PermissionEnum::Administrate];
    let admin_uid = try!(utils::check_valid_permission(
        perm_dao,
        user_dao,
        admin_key_path,
        &valid_permissions));

    // Load layout from file (and validate)

    // Create new fixtures, which create new channels

    // Return layout id

    Err(Error::TodoErr)
}

