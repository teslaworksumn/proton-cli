use rustc_serialize::json;
use std::path::Path;

use dao::{LayoutDao, PermissionDao, UserDao};
use error::Error;
use project_types::{FileLayout, Layout, PermissionEnum};
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

    // Load layout from file
    let layout_json = try!(utils::file_as_string(layout_path.as_ref()));
    let file_layout: FileLayout = try!(json::decode(&layout_json).map_err(Error::JsonDecode));
    let (layout, fixtures, channels) = try!(file_layout.as_layout());
    println!("layout: {:?}", layout);
    println!("fixtures: {:?}", fixtures);
    println!("channels: {:?}", channels);
    // Create new fixtures, which create new channels

    // Return layout id

    Err(Error::TodoErr)
}

