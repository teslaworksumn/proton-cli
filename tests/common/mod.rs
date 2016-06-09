// For some reason, Rust doesn't detect these functions being used in other tests
// Make the compiled output less verbose
#![allow(dead_code)]

extern crate proton_cli;
extern crate tempdir;
extern crate git2;

pub mod rsa_keys;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use tempdir::TempDir;
use self::git2::Repository;

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

/// Check that changes were actually committed to the repository
pub fn assert_repo_no_modified_files<P: AsRef<Path>>(repo_path: P) {
    let repo = Repository::open(&repo_path).unwrap();

    let commit = repo.refname_to_id("refs/heads/master")
        .and_then(|oid| repo.find_commit(oid))
        .expect("Finding master failed");
    let tree = commit.tree().expect("Opening master tree failed");

    let _ = repo.diff_tree_to_workdir_with_index(Some(&tree), None)
        .and_then(|diff| diff.stats())
        .map(|stats| {
            assert!(0 == stats.files_changed(), "No changes should be staged");
        });
}

/// Creates a PathBuf that points to the cli/tests directory
/// Needed because many tests change the current directory,
/// which makes env::current_dir() point to a temporary directory
/// that no longer exists instead of the present working directory (pwd)
///
/// Impure.
pub fn get_test_directory_path() -> PathBuf {
    // The first argument is the path to the executable (most of the time)
    // This gets us to /.../cli/target/debug/new_sequence-e77ba7396396e159 (or whatever)
    let exec_path_str = env::args().nth(0).expect("First argument not a valid path");
    // Now we work our way back up to cli/
    let mut test_dir_path = PathBuf::from(&exec_path_str);
    test_dir_path.pop();
    test_dir_path.pop();
    test_dir_path.pop();
    // and back down to cli/tests
    test_dir_path.push("tests");

    test_dir_path
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
