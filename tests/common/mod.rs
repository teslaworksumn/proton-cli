// For some reason, Rust doesn't detect these functions being used in other tests
// Make the compiled output less verbose
#![allow(dead_code)]

extern crate proton_cli;
extern crate tempdir;

pub mod rsa_keys;

use std::env;
use std::fs::File;
use std::io::{Write};
use std::path::{Path, PathBuf};

use tempdir::TempDir;

use proton_cli::{utils, Project, User};


/// Creates a key file at the given location
/// Returns the path to the key file
pub fn make_key_file<P: AsRef<Path>>(
    root_dir: P,
    file_name: &str,
    test_key: rsa_keys::TestKey
) -> PathBuf {

    let mut key_path = PathBuf::new();
    key_path.push(root_dir);
    key_path.push(file_name);

    let file_content = rsa_keys::get_test_key(test_key);
    File::create(&key_path)
        .and_then(|mut file| write!(file, "{}\n", file_content))
        .expect("Error creating key file");

    key_path
}

/// Check if the public key at the given path exists and contains key_content,
/// and check to see that the user is in the project at the current directory's protonfile
pub fn assert_user_added<P: AsRef<Path>>(public_key_path: P, name: &str) {
    let pub_key_contents = utils::file_as_string(public_key_path)
        .expect("Error reading public key file");

    let project: Project = utils::read_protonfile(None::<P>)
        .expect("Error reading project");
        
    let u = User {
        name: name.to_string(),
        public_key: pub_key_contents,
    };
    assert_eq!(project.user_exists(&u), true);
}

/// Creates a temporary directory to run a test out of
pub fn setup() -> TempDir {
    TempDir::new("proton_cli_tests").unwrap()
}

/// Creates a temporary directory, initializes a project in it,
/// and changes the current directory to it
/// Returns the path to the temp directory 
pub fn setup_init_cd() -> TempDir {
    let root_dir = setup();
    
    {
        let root = root_dir.path();

        let _ = proton_cli::initialize_project(&root)
            .expect("Error initializing project");

        // Move into temp directory (new_user assumes it is run in project directory)
        assert!(env::set_current_dir(&root).is_ok());
    }

    root_dir
}
