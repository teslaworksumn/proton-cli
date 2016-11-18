use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::env;

use git2::{self, Repository, Signature};
use openssl::rsa;
use openssl::pkey;
use rustc_serialize::json;

use dao::{PermissionDao, UserDao};
use project_types::{Project, PermissionEnum};
use error::Error;

/// Creates a new public/private key pair
pub fn create_pub_priv_keys() -> Result<(String, String), Error> {
    let keys = try!(rsa::Rsa::generate(2048).map_err(Error::Ssl));
    let pkey = try!(pkey::PKey::from_rsa(keys).map_err(Error::Ssl));
    let private_key = try!(pkey.private_key_to_pem().map_err(Error::Ssl));
    let public_key = try!(pkey.public_key_to_pem().map_err(Error::Ssl));
    let private_key_str = String::from_utf8(private_key).expect("Generated private key not UTF-8");
    let public_key_str = String::from_utf8(public_key).expect("Generated public key not UTF-8");
    Ok((public_key_str, private_key_str))
}

/// Checks if the user with a public key at the given path has
/// one of the given valid permissions
/// Returns this user if found and has permission, else error
pub fn check_valid_permission<P: AsRef<Path>, PD: PermissionDao, UD: UserDao>(
    perm_dao: &PD,
    user_dao: &UD,
    public_key_path: P,
    valid_permissions: &Vec<PermissionEnum>
) -> Result<u32, Error> {
    
    if valid_permissions.len() > 0 {
        let public_key = try!(file_as_string(public_key_path));
        let uid = try!(user_dao.get_user_id(&public_key));
        let permissions = try!(perm_dao.get_all_permissions(uid));
        for permission in permissions {
            if valid_permissions.contains(&permission.permission) {
                return Ok(uid);
            }
        }
    }

    Err(Error::UnauthorizedAction)
}

/// Returns the last part of the path, the file name, if no problems arise
/// Raises errors if the file name is invalid or cannot be converted to UTF-8
pub fn file_name_from_path<P: AsRef<Path>>(path: P) -> Result<String, Error> {
    match path.as_ref().file_name() {
        Some(name_os) => {
            match name_os.to_str() {
                Some(name) => Ok(name.to_string()),
                None => Err(Error::InvalidFileName),
            }
        },
        None => Err(Error::InvalidFileName),
    }
}

/// Stages all files in the repository and commits them
/// If a path to the repository is not given, assume it is the current directory
///
/// Impure.
pub fn commit_all<P: AsRef<Path>>(
    repo_path: Option<P>,
    signature: &Signature,
    msg: &str,
) -> Result<(), Error> {

    // Get repository and index
    let repo = try!(get_repo_from_path(repo_path));
    let mut index = try!(repo.index().map_err(Error::Git));

    // Add all modified files to the index
    let tree_oid = try!(index.add_all(vec!["."], git2::ADD_DEFAULT, None)
        .and_then(|_| index.write_tree())
        .map_err(Error::Git));

    let tree = try!(repo.find_tree(tree_oid).map_err(Error::Git));
    let parent = try!(repo.refname_to_id("refs/heads/master")
        .and_then(|oid| repo.find_commit(oid))
        .map_err(Error::Git));

    repo.commit(
        Some("HEAD"),
        signature,
        signature,
        msg,
        &tree,
        &[&parent]
    )
        .and_then(|_| index.write())
        .map_err(Error::Git)
}

/// Stages a file and commits it
/// If a path to the repository is not given, assume it is in the current directory
/// 
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

/// Creates a folder. The folder must not exist or must be empty.
///
/// Impure.
pub fn create_empty_directory<P: AsRef<Path>>(dir_path: P) -> Result<(), Error> {
    // Make the folder - ignore error.
    let _ = fs::create_dir(&dir_path);

    // Check that the folder is empty
    fs::read_dir(&dir_path)
        .map(|iter| iter.count())
        .map_err(Error::Io)
        .and_then(|count|
            if count == 0 {
                Ok(())
            } else {
                Err(Error::FolderNotEmpty(dir_path.as_ref().to_str().unwrap().to_owned(), count))
            })
}

/// Reads a Project from a Protonfile.
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

    Repository::open(repo_path.as_path())
        .map_err(Error::Git)
}