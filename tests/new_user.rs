extern crate proton_cli;
extern crate tempdir;

use std::env;
use std::fs::File;
use std::io::{Write};
use std::path::{Path, PathBuf};

use tempdir::TempDir;

use proton_cli::{Project, User, utils};


/// Warning: This test changes env::current_directory
/// to better model new_user's expected use case.
/// Running tests with RUST_TEST_THREADS=1 runs tests
/// in serial, which avoids occasional false negatives
#[test]
fn works_with_new_and_existing_protonfile() {
    // Make temp directory to work out of
    let root_dir = setup();
    let root = root_dir.path();

    // Make new project in temp directory
    let _ = proton_cli::initialize_project(&root)
        .expect("Error initializing project");

    // Make key files for users
    let key_path_a = make_key_file(root, "a.pub", "123");
    let key_path_b = make_key_file(root, "b.pub", "456");

    // Move into temp directory (new_user assumes it is run in project directory)
    assert!(env::set_current_dir(&root).is_ok());

    // Add new user to project
    let _ = proton_cli::new_user(&key_path_a.as_path(), String::from("Test User"))
        .expect("Error adding user");

    // Assert that user was added
    assert_exists(key_path_a.as_path(), "Test User", "123");

    // Now try adding another user
    let _ = proton_cli::new_user(&key_path_b.as_path(), String::from("Test User 2"))
        .expect("Error adding user 2");

    // Assert that both users exist
    assert_exists(key_path_a.as_path(), "Test User", "123");
    assert_exists(key_path_b.as_path(), "Test User 2", "456");
}

#[test]
#[should_panic(expected = "Error adding user")]
fn fails_with_a_nonexistent_protonfile() {
    let root_dir = setup();
    let root = root_dir.path();

    // Make key file, but don't initialize project
    let key_path = make_key_file(root, "a.pub", "123");

    match proton_cli::new_user(&key_path.as_path(), String::from("Username")) {
        Ok(_) => (),
        Err(_) => panic!("Error adding user"),
    };
}

#[test]
#[should_panic(expected = "Error adding user")]
fn fails_with_nonexistent_key_path() {
    let root_dir = setup();
    let root = root_dir.path();

    let _ = proton_cli::initialize_project(&root)
        .expect("Error initializing project");
    
    let key_path = root.join("nonexistent");

    // Move into temp directory (new_user assumes it is run in project directory)
    assert!(env::set_current_dir(&root).is_ok());

    match proton_cli::new_user(&key_path.as_path(), String::from("Username")) {
        Ok(_) => (),
        Err(_) => panic!("Error adding user"),
    };
}

/// Warning: This test changes env::current_directory
/// to better model new_user's expected use case.
/// Running tests with RUST_TEST_THREADS=1 runs tests
/// in serial, which avoids occasional false negatives
#[test]
#[should_panic(expected = "Error adding user 2")]
fn fails_with_duplicate_user_key() {
    let root_dir = setup();
    let root = root_dir.path();

    let _ = proton_cli::initialize_project(&root)
        .expect("Error initializing project");
    
    let key_path = make_key_file(root, "a.pub", "123");

    // Move into temp directory (new_user assumes it is run in project directory)
    assert!(env::set_current_dir(&root).is_ok());

    // Add new user to project
    let _ = proton_cli::new_user(&key_path.as_path(), String::from("Test User"))
        .expect("Error adding user");

    // Assert that user was added
    assert_exists(key_path.as_path(), "Test User", "123");

    // Now try adding another user with the same key
    let _ = proton_cli::new_user(&key_path.as_path(), String::from("Test User 2"))
        .expect("Error adding user 2");

    panic!("Should not get to here");
}

/// Warning: This test changes env::current_directory
/// to better model new_user's expected use case.
/// Running tests with RUST_TEST_THREADS=1 runs tests
/// in serial, which avoids occasional false negatives
#[test]
#[should_panic(expected = "Error adding second user")]
fn fails_with_duplicate_user_name() {
    let root_dir = setup();
    let root = root_dir.path();

    let _ = proton_cli::initialize_project(&root)
        .expect("Error initializing project");
    
    let key_path_a = make_key_file(root, "a.pub", "123");
    let key_path_b = make_key_file(root, "b.pub", "456");

    // Move into temp directory (new_user assumes it is run in project directory)
    assert!(env::set_current_dir(&root).is_ok());

    // Add new user to project
    let _ = proton_cli::new_user(&key_path_a.as_path(), String::from("Test User"))
        .expect("Error adding user");

    // Assert that user was added
    assert_exists(key_path_a.as_path(), "Test User", "123");

    // Now try adding another user with the same key
    let _ = proton_cli::new_user(&key_path_b.as_path(), String::from("Test User"))
        .expect("Error adding second user");

    panic!("Should not get to here");
}

/// Check if the public key at the given path exists and contains key_content,
/// and check to see that the user is in the project at the current directory's protonfile
fn assert_exists<P: AsRef<Path>>(public_key_path: P, name: &str, key_content: &str) {
    let pub_key_contents = utils::file_as_string(public_key_path)
        .expect("Error reading public key file");

    assert_eq!(pub_key_contents.trim(), key_content.trim());

    let project: Project = utils::read_protonfile(None::<P>)
        .expect("Error reading project");
        
    let u = User {
        name: name.to_string(),
        public_key: key_content.to_string(),
    };
    assert_eq!(project.user_exists(&u), true);
}

/// Creates a key file at the given location
/// Returns the path to the key file
fn make_key_file<P: AsRef<Path>>(root_dir: P, file_name: &str, file_content: &str) -> PathBuf {
    let mut key_path = PathBuf::new();
    key_path.push(root_dir);
    key_path.push(file_name);
    File::create(&key_path)
        .and_then(|mut file| write!(file, "{}\n", file_content))
        .expect("Error creating key file");

    key_path
}

/// Creates a temporary directory to run a test out of
fn setup() -> TempDir {
    TempDir::new("proton_cli_tests").unwrap()
}
