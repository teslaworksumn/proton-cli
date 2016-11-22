//! This module initializes a project.

use std::path::Path;

use utils;
use error::Error;
use dao::{LayoutDao, PermissionDao, ProjectDao, UserDao};


/// Initializes a new project at root. The root must either not exist, or must
/// be an empty directory. This will
///
/// 1. Create the directory if it doesn't exist.
/// 2. Create a Protonfile
/// 3. Initialize a git repository and commit the protonfile.
///
/// Impure.
pub fn new_project<LD: LayoutDao, PMD: PermissionDao, PTD: ProjectDao, UD: UserDao>(
    layout_dao: &LD,
    perm_dao: &PMD,
    project_dao: &PTD,
    user_dao: &UD,
    name: &str,
    layout_id: u32
) -> Result<String, Error> {

    // Check that layout exists
    let _ = try!(layout_dao.get_layout(layout_id));

    // Create keys
    let (root_pub_key, root_private_key) = try!(utils::create_pub_priv_keys());

    // Add project root user
    let root_uid = try!(user_dao.add_initial_user(name, &root_private_key, &root_pub_key));

    // Give initial user admin permissions
    try!(perm_dao.add_initial_permission(root_uid));

    // Create new project
    let _ = try!(project_dao.new_project(name, layout_id));

    // Return root user's public key
    Ok(root_pub_key)
}
