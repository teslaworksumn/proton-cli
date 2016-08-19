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
            "editseq" => {
                let sequence_name = target_sequence.unwrap();
                let project = try!(utils::read_protonfile(None::<&Path>));
                if project.find_sequence_by_name(&sequence_name).is_none() {
                    Err(Error::SequenceNotFound(sequence_name))
                } else {
                    Ok(Permission::EditSeq(sequence_name))
                }
            },
            "editseqsec" => {
                let sequence_name = target_sequence.unwrap();
                let section_idx = target_section.unwrap();
                let project = try!(utils::read_protonfile(None::<&Path>));
                let sequence_opt = project.find_sequence_by_name(&sequence_name);
                if sequence_opt.is_none() {
                    Err(Error::SequenceNotFound(sequence_name))
                } else {
                    let sequence = sequence_opt.unwrap().to_owned();
                    if !sequence.section_in_range(section_idx) {
                        println!("EditSeqSec target section not in range");
                        Err(Error::InvalidSequenceSection(section_idx))
                    } else {
                        Ok(Permission::EditSeqSec(sequence_name, section_idx))
                    }
                }
            },
            _ => Err(Error::InvalidPermissionName(perm_name.to_owned()))
        }
    }
}
