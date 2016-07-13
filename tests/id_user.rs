extern crate proton_cli;
extern crate tempdir;

mod common;

use std::path::Path;

use common::rsa_keys::TestKey;
use common::setup;
use proton_cli::project_types::User;


fn try_id_user(
    root_path: &Path,
    name: &str,
    public_key: TestKey,
    private_key: TestKey
) {

    // Add user to project
    let root_private_key_path = common::make_key_file(root_path, "root.pem", TestKey::RootKeyPem);

    setup::try_new_user(
        &root_private_key_path.as_path(),
        root_path,
        &name,
        "a.pub",
        public_key.clone());

    // Make private key file for user
    let private_key_path = common::make_key_file(root_path, "a.pem", private_key);

    // Identify user
    let user = proton_cli::id_user(&private_key_path.as_path())
        .expect("Error identifying user");

    // Assert that the user created was the one identified
    let public_key_str = common::rsa_keys::get_test_key(public_key);
    assert_user_equal(&user, &name, &public_key_str.trim_right());
}

fn assert_user_equal(user: &User, name: &str, pub_key: &str) {
    let u = User::new(name, &pub_key).expect("Creating user failed");
    assert_eq!(user, &u);
}

#[test]
fn works_with_valid_keys() {
    let root = setup::setup_init_cd();
    let name = "Test User";

    try_id_user(root.path(), &name, TestKey::GoodKeyPub, TestKey::GoodKeyPem);

}

#[test]
#[should_panic(expected = "No such file or directory")]
fn fails_with_nonexistent_private_key() {
    let root = setup::setup_init_cd();
    let name = "Test User".to_string();

    // Add user to project
    let root_private_key_path = common::make_key_file(root.path(), "root.pem", TestKey::RootKeyPem);

    setup::try_new_user(
        root_private_key_path.as_path(),
        root.path(),
        &name,
        "a.pub",
        TestKey::GoodKeyPub);

    // Make bad path to private key file
    let private_key_path = root.path().join("nonexistent.pem");

    proton_cli::id_user(&private_key_path.as_path())
        .expect("Error identifying user");
}

#[test]
#[should_panic(expected = "UserNotFound")]
fn fails_with_valid_private_key_no_match() {
    let root = setup::setup_init_cd();
    let name = "Test User".to_string();

    // Add user to project
    let root_private_key_path = common::make_key_file(root.path(), "root.pem", TestKey::RootKeyPem);

    setup::try_new_user(
        root_private_key_path.as_path(),
        root.path(),
        &name,
        "a.pub",
        TestKey::GoodKeyPub);

    // Make private key for user
    let private_key_path = common::make_key_file(root.path(), "a.pem", TestKey::GoodKey2Pem);

    proton_cli::id_user(&private_key_path.as_path())
        .expect("Error identifying user");
}

#[test]
#[should_panic(expected = "Ssl")]
fn fails_with_invalid_private_key() {
    let root = setup::setup_init_cd();
    let name = "Test User".to_string();

    let root_private_key_path = common::make_key_file(root.path(), "root.pem", TestKey::RootKeyPem);

    setup::try_new_user(
        root_private_key_path.as_path(),
        root.path(),
        &name,
        "a.pub",
        TestKey::GoodKeyPub);

    // Make bad private key for user
    let private_key_path = common::make_key_file(root.path(), "a.pem", TestKey::BadPrivKeyPem);

    // Identify user
    proton_cli::id_user(&private_key_path.as_path())
        .expect("Error identifying user");
}
