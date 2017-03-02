use std::path::Path;

use error::Error;
use project_types::Permission;
use dao::PermissionDao;


/// [INCOMPLETE] Gets the permissions a user has
pub fn get_permissions<P: AsRef<Path>, PD: PermissionDao> (pdao: PD, uid: u32
) -> Result<Vec<Permission>, Error> {
    pdao.get_all_permissions(uid)
}

/// [INCOMPLETE] Sets a user's permission
#[allow(unused_variables)]
pub fn set_permission<P: AsRef<Path>> (
    admin_key_path: P,
    add: bool,
    target_uid: u32,
    permission_name: &str,
    target_sequence: Option<u32>,
    target_section: Option<u32>
) -> Result<(), Error> {
    //let admin_uid = try!(utils::get_uid_from_key(&admin_key_path));

    Err(Error::TodoErr)
    
    // Only admins (those with GrantPerm permission) can change permissions
    // Make sure root isn't losing admin privileges
    // Validate and create permission
    // Set permissions
    // Commit changes
    
}

