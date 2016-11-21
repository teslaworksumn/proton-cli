use rustc_serialize::json;
use std::path::Path;

use dao::{ChannelDao, FixtureDao, LayoutDao, PermissionDao, UserDao};
use error::Error;
use project_types::{FileLayout, Layout, PermissionEnum};
use utils;

pub fn new_layout<P: AsRef<Path>, CD: ChannelDao, FD: FixtureDao, LD: LayoutDao, PD: PermissionDao, UD: UserDao>(
    chan_dao: &CD,
    fix_dao: &FD,
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
    
    // Make sure layout is valid
    try!(file_layout.validate());

    // Create new channels and fixtures from layout and add to storage
    let (channels, fixtures) = try!(file_layout.create_new_parts(chan_dao, fix_dao));

    // Create new layout from fixtures
    let fix_ids = fixtures.iter()
        .map(|fixture| fixture.fixid)
        .collect::<Vec<u32>>();
    let layout = try!(layout_dao.new_layout(&file_layout.layoutName, fix_ids));

    // Return layout id
    Ok(layout.layout_id)
}

