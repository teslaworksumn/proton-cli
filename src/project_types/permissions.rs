use std::path::Path;
use std::ascii::AsciiExt;

use error::Error;
use dao::PermissionDao;
use utils;


#[derive(Debug, RustcDecodable, RustcEncodable)]
pub enum PermissionEnum {
    Administrate,
    EditSequence,
    EditSection,
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct Permission {
    pub uid: u32,
    pub seqid: u32,
    pub secid: u32,
    pub permission: PermissionEnum,
}

impl Permission {
    /// Creates a new Permission
    pub fn new(
        uid: u32,
        seqid: Option<u32>,
        secid: Option<u32>,
        perm: PermissionEnum
    ) -> Result<Permission, Error> {
        // Make sure user exists
        // Check seqid and secid existence based on perm

        Err(Error::TodoErr)
    }

    /// Add a user permission
    pub fn add_permission<T: PermissionDao>(dao: &T, perm: Permission) -> Result<(), Error> {
        Err(Error::TodoErr)
    }

}
