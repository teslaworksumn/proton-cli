
use std::path::Path;
use std::ascii::AsciiExt;

use git2::Signature;

use error::Error;
use project_types::{User, Permission};
use utils;
use user;


pub fn set_permission(
    auth_user: &User,
    add: bool,
    target_username: &str,
    permission_name: &str,
    target_sequence: Option<String>,
    target_section: Option<u32>
) -> Result<(), Error> {

    // Only admins (those with GrantPerm permission) can change permissions
    if !auth_user.is_admin() {
        return Err(Error::UnauthorizedAction);
    }

    // Make sure root isn't losing admin privileges
    if target_username == "root" && !add && permission_name.eq_ignore_ascii_case("Administrate") {
        return Err(Error::UnauthorizedAction);
    }

    // Validate and create permission
    let perm = try!(Permission::new(permission_name, target_sequence, target_section));

    // Get project that will be modified
    let mut project = try!(utils::read_protonfile(None::<&Path>));

    // Set permissions
    try!(project.set_user_permission(&target_username, perm.to_owned(), add));

    // Save changes to protonfile
    try!(utils::write_protonfile(&project, None::<&Path>));

    // Commit changes
    let signature = Signature::now(&auth_user.name, "proton@teslaworks.net").unwrap();
    let change_type = match add {
        true => "granting",
        false => "revoking",
    };
    let msg = format!("Admin '{}' {} permission '{:?}' to/from user {}",
        auth_user.name, change_type, perm, target_username);

    utils::commit_all(None::<&Path>, &signature, &msg)
}

pub fn get_permissions<P: AsRef<Path>> (user_key_path: P
) -> Result<Vec<Permission>, Error> {
    user::id_user(&user_key_path).map(|user| user.permissions)
}
