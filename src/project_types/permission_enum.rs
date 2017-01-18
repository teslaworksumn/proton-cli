use error::Error;

#[derive(Debug, PartialEq, RustcDecodable, RustcEncodable)]
pub enum PermissionEnum {
    Administrate,
    EditSequence(u32),
    EditSection(u32, u32),
}

/// Gets a permission enum from a string if valid
pub fn get_permission_enum(
    s: &str,
    seqid: Option<u32>,
    secid: Option<u32>
) -> Result<PermissionEnum, Error> {

    match s {
        "Administrate" => Ok(PermissionEnum::Administrate),
        "EditSequence" => match seqid {
            Some(seq) => Ok(PermissionEnum::EditSequence(seq)),
            None => Err(Error::MissingPermissionArg)
        },
        "EditSection" => match seqid {
            Some(seq) => match secid {
                Some(sec) => Ok(PermissionEnum::EditSection(seq, sec)),
                None => Err(Error::MissingPermissionArg),
            },
            None => Err(Error::MissingPermissionArg),
        },
        x => Err(Error::InvalidPermissionName(x.to_owned()))
    }
}
