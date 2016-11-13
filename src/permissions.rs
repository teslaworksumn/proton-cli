
use std::path::Path;
use std::ascii::AsciiExt;

use git2::Signature;

use error::Error;
use project_types::{User, Permission};
use utils;
use user;
use dao::{PermissionDao, UserDao};


pub fn get_permissions<P: AsRef<Path>, PD: PermissionDao> (pdao: PD, uid: u32
) -> Result<Vec<Permission>, Error> {
    pdao.get_all_permissions(uid)
}

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
    
    // let signature = Signature::now(&auth_user.name, "proton@teslaworks.net").unwrap();
    // let change_type = match add {
    //     true => "granting",
    //     false => "revoking",
    // };
    // let msg = format!("Admin '{}' {} permission '{:?}' to/from user {}",
    //     auth_user.name, change_type, perm, target_username);

    // utils::commit_all(None::<&Path>, &signature, &msg)
}

