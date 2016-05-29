use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::env;

use rustc_serialize::json;
use git2::{Repository, Signature};

use project_types::Project;
use error::Error;


/// Stages a file and commits it
/// If a path to the repository is not given, assume it is in the current directory
/// Impure.
pub fn commit_file<P: AsRef<Path>>(
    file_path: &Path, repo_path: Option<P>, signature: &Signature, msg: &str
) -> Result<(), Error> {
    
    let repo = try!(get_repo_from_path(repo_path));
    let tree_oid = try!(repo.index()
        .and_then(|mut index| index.add_path(&file_path).map(|_| index))
        .and_then(|mut index| index.write().map(|_| index))
        .and_then(|mut index| index.write_tree())
        .map_err(Error::Git));
    let tree = try!(repo.find_tree(tree_oid).map_err(Error::Git));
    let parent = try!(repo.refname_to_id("refs/heads/master")
        .and_then(|oid| repo.find_commit(oid))
        .map_err(Error::Git));

    return repo.commit(
        Some("HEAD"),
        signature,
        signature,
        msg,
        &tree,
        &[&parent]
    )
        .map_err(Error::Git)
        .map(|_| ())
    
}

/// Reads a Project from a Protonfile.
/// Wraps any errors in proton_cli::Error
/// Assumes Protonfile.json resides in the current directory
/// unless a path to the Protonfile is given.
pub fn read_protonfile<P: AsRef<Path>>(pf_path: Option<P>) -> Result<Project, Error> {
    let protonfile_path = build_protonfile_path(pf_path);
    let protonfile = try!(file_as_string(&protonfile_path));
    json::decode(&protonfile).map_err(Error::JsonDecode)
}

/// Saves a Project to a Protonfile.
/// Assumes the Protonfile is in the current directory
/// unless a path to the Protonfile is given.
/// Impure.
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

fn get_repo_from_path<P: AsRef<Path>>(path_opt: Option<P>) -> Result<Repository, Error> {
    let mut repo_path = PathBuf::new();
    let _ = match path_opt {
        Some(path) => repo_path.push(path),
        None => repo_path.push(env::current_dir().expect("Current directory invalid")),
    };

    println!("{}", repo_path.as_path().display());

    Repository::open(repo_path.as_path())
        .map_err(Error::Git)

}