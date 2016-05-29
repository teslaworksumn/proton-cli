extern crate proton_cli;
extern crate tempdir;

mod common;

use std::path::PathBuf;

use common::rsa_keys::TestKey;
use proton_cli::{User, utils};


#[test]
fn works_with_valid_keys() {
    let root = common::setup_init_cd();

    // Make key files for users
    let public_key_path = common::make_key_file(root.path(), "a.pub", TestKey::GoodKeyPub);
    let private_key_path = common::make_key_file(root.path(), "a.pem", TestKey::GoodKeyPem);

    let name = "Test User".to_string();

    // Add new user to project
    let _ = proton_cli::new_user(&public_key_path.as_path(), &name)
        .expect("Error adding user");

    // Assert that user was added
    common::assert_user_added(public_key_path.as_path(), &name);

    // Identify user
    let user = proton_cli::id_user(&private_key_path.as_path())
        .expect("Error identifying user");

    // Assert equality
    assert_user_equal(&user, &name, public_key_path);
}

#[test]
#[should_panic(expected = "IO error occurred")]
fn fails_with_nonexistent_private_key() {
    let root = common::setup_init_cd();

    // Make key files for users
    let public_key_path = common::make_key_file(root.path(), "a.pub", TestKey::GoodKeyPub);
    let private_key_path = root.path().join("nonexistent.pem");

    let name = "Test User".to_string();

    // Add new user to project
    let _ = proton_cli::new_user(&public_key_path.as_path(), &name)
        .expect("Error adding user");

    // Assert that user was added
    common::assert_user_added(public_key_path.as_path(), &name);

    // Identify user
    match proton_cli::id_user(&private_key_path.as_path()) {
        Ok(_) => (),
        Err(e) => panic!(e.to_string()),
    }
}

#[test]
#[should_panic(expected = "User not found")]
fn fails_with_valid_private_key_no_match() {
    let root = common::setup_init_cd();

    // Make key files for users
    let public_key_path = common::make_key_file(root.path(), "a.pub", TestKey::GoodKeyPub);
    let private_key_path = common::make_key_file(root.path(), "a.pem", TestKey::GoodKey2Pem);

    let name = "Test User".to_string();

    // Add new user to project
    let _ = proton_cli::new_user(&public_key_path.as_path(), &name)
        .expect("Error adding user");

    // Assert that user was added
    common::assert_user_added(public_key_path.as_path(), &name);

    // Identify user
    match proton_cli::id_user(&private_key_path.as_path()) {
        Ok(_) => (),
        Err(e) => panic!(e.to_string()),
    }
}

#[test]
#[should_panic(expected = "SSL error occured")]
fn fails_with_invalid_private_key() {
    let root = common::setup_init_cd();

    // Make key files for users
    let public_key_path = common::make_key_file(root.path(), "a.pub", TestKey::GoodKeyPub);
    let private_key_path = common::make_key_file(root.path(), "a.pem", TestKey::BadPrivKeyPem);

    let name = "Test User".to_string();

    // Add new user to project
    let _ = proton_cli::new_user(&public_key_path.as_path(), &name)
        .expect("Error adding user");

    // Assert that user was added
    common::assert_user_added(public_key_path.as_path(), &name);

    // Identify user
    match proton_cli::id_user(&private_key_path.as_path()) {
        Ok(_) => (),
        Err(e) => panic!(e.to_string()),
    }
}

fn assert_user_equal(user: &User, name: &str, pub_key_path: PathBuf) {
    let pub_key = utils::file_as_string(pub_key_path).expect("Error reading public key");
    let u = User {
        name: name.to_string(),
        public_key: pub_key,
    };
    assert_eq!(user, &u);
}
