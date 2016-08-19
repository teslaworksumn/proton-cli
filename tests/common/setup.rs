extern crate tempdir;

use std::env;
use std::path::{Path, PathBuf};

use self::tempdir::TempDir;

use proton_cli;
use proton_cli::project_types::{Sequence, User};

use super::rsa_keys::{self, TestKey};


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
        let root_pub_key = rsa_keys::get_test_key(TestKey::RootKeyPub);

        let _ = proton_cli::initialize_project(root, &root_pub_key)
            .expect("Error initializing project");

        // Move into temp directory (new_user assumes it is run in project directory)
        assert!(env::set_current_dir(&root).is_ok());
    }

    root_dir
}

/// Creates a key file for a new user,
/// then tries to add the user to the project
pub fn try_new_user(
    admin_key_path: &Path,
    root_path: &Path,
    user_name: &str,
    key_name: &str,
    key: TestKey
) -> User {

    let user_key_path = super::make_key_file(&root_path, &key_name, key);

    let _ = proton_cli::new_user(&admin_key_path, &user_key_path.as_path(), &user_name)
        .expect("Error creating new user");

    super::assert_user_added(user_key_path.as_path(), &user_name);

    super::assert_repo_no_modified_files(&root_path);

    let project = proton_cli::utils::read_protonfile(None::<&Path>).expect("Error reading project");
    project.find_user_by_name(&user_name).unwrap().to_owned()
}

/// Attempts to make a new sequence with the given name and music file
pub fn try_make_sequence(
    admin_key_path: &Path,
    name: &str,
    music_file: &str
) -> Sequence {
    let music_file_path = super::get_music_file_path(music_file);

    let _ = proton_cli::new_sequence(&admin_key_path, &name, &music_file_path.as_path())
        .expect("Error creating sequence");

    let project = proton_cli::utils::read_protonfile(None::<&Path>)
        .expect("Error reading project from file");

    let found_sequence = project.find_sequence_by_name(name);

    assert!(found_sequence.is_some());
    let seq = found_sequence.unwrap();

    assert!(seq.num_sections == 1);
    let mut seq_dir_path = PathBuf::from(&seq.directory_name);
    
    assert!(seq_dir_path.exists());
    seq_dir_path.push(&seq.music_file_name);
    assert!(seq_dir_path.exists());

    seq.to_owned()
}

/// Tries to modify a user's permission
/// Panics on error
///
/// Impure.
pub fn try_set_permission<P: AsRef<Path>>(
    root_path: &Path,
    auth_private_key_path: P,
    add: bool,
    target_username: &str,
    permission_name: &str,
    target_sequence: Option<String>,
    target_section: Option<u32>
) {
    let auth_user = proton_cli::id_user(&auth_private_key_path)
        .expect("Auth user not found");

    proton_cli::set_permission(
        &auth_user,
        add,
        &target_username,
        permission_name,
        target_sequence,
        target_section
    ).expect("Error setting permission");

    super::assert_repo_no_modified_files(&root_path);
}
