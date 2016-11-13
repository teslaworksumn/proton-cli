use error::Error;


#[derive(Debug, RustcDecodable, RustcEncodable)]
pub enum PermissionEnum {
    Administrate,
    EditSequence,
    EditSection,
}

/// Gets a permission enum from a string if valid
pub fn get_permission_enum(s: &str) -> Result<PermissionEnum, Error> {
    match s {
        "Administrate" => Ok(PermissionEnum::Administrate),
        "EditSequence" => Ok(PermissionEnum::EditSequence),
        "EditSection" => Ok(PermissionEnum::EditSection),
        x => Err(Error::InvalidPermissionName(x.to_owned()))
    }
}
