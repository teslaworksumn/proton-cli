extern crate proton_cli;
extern crate tempdir;
extern crate git2;

mod common;

use std::path::{Path, PathBuf};

use common::setup;
use common::rsa_keys::TestKey;


#[test]
fn works_with_valid_key_and_name() {
    let root = setup::setup_init_cd();
    let root_key_path = common::make_key_file(&root.path(), "root.pem", TestKey::RootKeyPem);

    let _ = setup::try_make_sequence(&root_key_path.as_path(), "asdf", "Dissonance.ogg");

    proton_cli::remove_sequence(&root_key_path.as_path(), "asdf")
        .expect("Error removing sequence");

    let project = proton_cli::utils::read_protonfile(None::<&Path>)
        .expect("Error reading project from file");

    let found_sequence = project.find_sequence_by_name("asdf");

    assert!(found_sequence.is_none());

    let mut sequence_dir_path = PathBuf::from(root.path());
    sequence_dir_path.push("seq_asdf");
    assert!(!sequence_dir_path.exists());

    common::assert_repo_no_modified_files(&root.path());
}

#[test]
#[should_panic(expected = "Error removing sequence: Ssl")]
fn fails_with_invalid_admin_key() {
    let root = setup::setup_init_cd();
    let root_key_path = common::make_key_file(&root.path(), "root.pem", TestKey::RootKeyPem);
    let root_key_bad_path = common::make_key_file(&root.path(), "root_bad.pub", TestKey::RootKeyPub);

    let _ = setup::try_make_sequence(&root_key_path.as_path(), "asdf", "Dissonance.ogg");
    proton_cli::remove_sequence(&root_key_bad_path.as_path(), "asdf")
        .expect("Error removing sequence");
}

#[test]
#[should_panic(expected = "Error removing sequence: Io")]
fn fails_with_nonexistent_admin_key() {
    let root = setup::setup_init_cd();
    let root_key_path = common::make_key_file(&root.path(), "root.pem", TestKey::RootKeyPem);
    let root_key_bad_path = PathBuf::from("nonexistent");

    let _ = setup::try_make_sequence(&root_key_path.as_path(), "asdf", "Dissonance.ogg");
    proton_cli::remove_sequence(&root_key_bad_path.as_path(), "asdf")
        .expect("Error removing sequence");
}

#[test]
#[should_panic(expected = "Error removing sequence: UnauthorizedAction")]
fn fails_with_unauthorized_admin_key() {
    let root = setup::setup_init_cd();
    let root_key_path = common::make_key_file(&root.path(), "root.pem", TestKey::RootKeyPem);
    let normal_key_path = common::make_key_file(&root.path(), "normal.pem", TestKey::GoodKeyPem);

    setup::try_new_user(
        &root_key_path.as_path(),
        root.path(),
        &"Test User",
        "normal.pub",
        TestKey::GoodKeyPub);

    let _ = setup::try_make_sequence(&root_key_path.as_path(), "asdf", "Dissonance.ogg");
    proton_cli::remove_sequence(&normal_key_path.as_path(), "asdf")
        .expect("Error removing sequence");
}

#[test]
#[should_panic(expected = "Error removing sequence: SequenceNotFound")]
fn fails_with_nonexistent_sequence_name() {
    let root = setup::setup_init_cd();
    let root_key_path = common::make_key_file(&root.path(), "root.pem", TestKey::RootKeyPem);

    let _ = setup::try_make_sequence(&root_key_path.as_path(), "asdf", "Dissonance.ogg");
    proton_cli::remove_sequence(&root_key_path.as_path(), "a")
        .expect("Error removing sequence");
}

#[test]
#[should_panic(expected = "Error removing sequence: InvalidSequenceName")]
fn fails_with_bad_sequence_name() {
    let root = setup::setup_init_cd();
    let root_key_path = common::make_key_file(&root.path(), "root.pem", TestKey::RootKeyPem);

    let _ = setup::try_make_sequence(&root_key_path.as_path(), "asdf", "Dissonance.ogg");
    proton_cli::remove_sequence(&root_key_path.as_path(), "as df")
        .expect("Error removing sequence");
}
