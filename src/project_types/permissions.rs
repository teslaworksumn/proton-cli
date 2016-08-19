use std::path::Path;
use std::ascii::AsciiExt;

use error::Error;
use utils;


#[derive(Clone, Debug, Eq, PartialEq, RustcEncodable, RustcDecodable)]
pub enum Permission {
    Administrate,
    EditSeq(String),
    EditSeqSec(String, u32),
}

impl Permission {
    /// Creates a new Permission
    /// Assumes target options are from Docopt, and are therefore safe to unwrap
    /// if the permission name is correct
    pub fn new(
        perm_name: &str,
        target_sequence: Option<String>,
        target_section: Option<u32>
    ) -> Result<Permission, Error> {

        match perm_name.to_ascii_lowercase().as_ref() {
            "administrate" => Ok(Permission::Administrate),
            "editseq" => Ok(Permission::EditSeq(target_sequence.unwrap())),
            "editseqsec" => Ok(Permission::EditSeqSec(
                target_sequence.unwrap(),
                target_section.unwrap())),
            _ => Err(Error::InvalidPermissionName(perm_name.to_owned()))
        }
    }
}
