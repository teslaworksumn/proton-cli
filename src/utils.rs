use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use rustc_serialize::json;

use project_types::Project;
use error::Error;


/// Reads a Project from a Protonfile.
/// Wraps any errors in proton_cli::Error
/// Assumes Protonfile.json resides in the current directory
/// unless a path to the Protonfile is given
pub fn read_protonfile<P: AsRef<Path>>(pf_path: Option<P>) -> Result<Project, Error> {
    let protonfile_path = build_protonfile_path(pf_path);
    let protonfile = try!(file_as_string(&protonfile_path));
    json::decode(&protonfile).map_err(Error::JsonDecode)
}

/// Saves a Project to a Protonfile.
/// Assumes the Protonfile is in the current directory
/// unless a path to the Protonfile is given
pub fn write_protonfile<P: AsRef<Path>>(project: &Project, pf_path: Option<P>) -> Result<(), Error> {
    let pretty_json = json::as_pretty_json(&project);
    let protonfile_path = build_protonfile_path(pf_path);
    File::create(&protonfile_path)
        .and_then(|mut protonfile| write!(protonfile, "{}\n", pretty_json))
        .map_err(Error::Io)
}

/// Reads a file as a string.
/// Wraps Read::read_to_string errors in proton_cli::Error
pub fn file_as_string<P: AsRef<Path>>(path: P) -> Result<String, Error> {
    File::open(path)
        .and_then(|mut file| {
            let mut string = String::new();
            file.read_to_string(&mut string)
                .and_then(|_| Ok(string.trim().to_string()))           
        })
        .map_err(Error::Io)
}

fn build_protonfile_path<P: AsRef<Path>>(path_opt: Option<P>) -> PathBuf {
    let mut protonfile_path = PathBuf::new();
    let _ = match path_opt {
        Some(path) => protonfile_path.push(path),
        None => (),
    };
    protonfile_path.push("Protonfile.json");
    protonfile_path
}