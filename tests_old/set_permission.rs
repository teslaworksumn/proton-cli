extern crate proton_cli;

mod common;

use std::path::Path;

use common::setup;
use common::rsa_keys::TestKey;


#[test]
#[allow(unused_variables)]
// root reference must be kept to keep temp directory in scope, but is never used
fn works_with_administrate() {
    let root = setup::setup_init_cd();
    let root_private_key_path = common::make_key_file(root.path(), "root.pem", TestKey::RootKeyPem);

    // Create user
    setup::try_new_user(
        &root_private_key_path.as_path(),
        root.path(),
        "Test User",
        "a.pub",
        TestKey::GoodKeyPub);

    // Try to add permission to user
    setup::try_set_permission(
        &root.path(),
        &root_private_key_path,
        true,
        "Test User",
        "Administrate",
        None::<String>,
        None::<u32>);

    // Now try to remove the permission
    setup::try_set_permission(
        &root.path(),
        &root_private_key_path,
        false,
        "Test User",
        "Administrate",
        None::<String>,
        None::<u32>);

}

#[test]
#[allow(unused_variables)]
// root reference must be kept to keep temp directory in scope, but is never used
fn works_with_editseq() {
    let root = setup::setup_init_cd();
    let root_private_key_path = common::make_key_file(root.path(), "root.pem", TestKey::RootKeyPem);

    // Create user
    setup::try_new_user(
        root_private_key_path.as_path(),
        root.path(),
        "Test User",
        "a.pub",
        TestKey::GoodKeyPub);

    // Create sequence
    let _ = setup::try_make_sequence(&root_private_key_path, "test_seq", "Dissonance.ogg");

    // Try to add permission to user
    setup::try_set_permission(
        &root.path(), 
        &root_private_key_path,
        true,
        "Test User",
        "EditSeq",
        Some("test_seq".to_owned()),
        None::<u32>);

    // Now try removing the permission
    setup::try_set_permission(
        &root.path(), 
        &root_private_key_path,
        false,
        "Test User",
        "EditSeq",
        Some("test_seq".to_owned()),
        None::<u32>);
}

#[test]
#[allow(unused_variables)]
// root reference must be kept to keep temp directory in scope, but is never used
fn works_with_editseqsec() {
    let root = setup::setup_init_cd();
    let root_private_key_path = common::make_key_file(root.path(), "root.pem", TestKey::RootKeyPem);

    // Create user
    let user = setup::try_new_user(
        root_private_key_path.as_path(),
        root.path(),
        "Test User",
        "a.pub",
        TestKey::GoodKeyPub);

    // Create sequence
    let _ = setup::try_make_sequence(&root_private_key_path.as_path(), "test_seq", "Dissonance.ogg");

    // Try to add permission to user
    setup::try_set_permission(
        &root.path(), 
        &root_private_key_path,
        true,
        "Test User",
        "EditSeqSec",
        Some("test_seq".to_owned()),
        Some(0));

    // Now try removing the permission
    setup::try_set_permission(
        &root.path(), 
        &root_private_key_path,
        false,
        "Test User",
        "EditSeqSec",
        Some("test_seq".to_owned()),
        Some(0));
}

#[test]
#[should_panic(expected = "UnauthorizedAction")]
fn fails_removing_root_admin() {
    let root = setup::setup_init_cd();
    let root_private_key_path = common::make_key_file(root.path(), "root.pem", TestKey::RootKeyPem);
    let admin2_private_key_path = common::make_key_file(root.path(), "b.pem", TestKey::GoodKeyPem);

    // Setup new user with Administrate permission
    setup::try_new_user(
        &root_private_key_path.as_path(),
        root.path(),
        "Admin2",
        "b.pub",
        TestKey::GoodKeyPub);
    
    setup::try_set_permission(
        &root.path(),
        &root_private_key_path,
        true,
        "Admin2",
        "Administrate",
        None::<String>,
        None::<u32>);

    // Now have that new user take away root's Administrate permission
    setup::try_set_permission(
        &root.path(),
        &admin2_private_key_path,
        false,
        "root",
        "Administrate",
        None::<String>,
        None::<u32>);
}

#[test]
#[should_panic(expected = "SequenceNotFound")]
fn fails_with_bad_target_editseq() {
    let root = setup::setup_init_cd();
    let root_private_key_path = common::make_key_file(root.path(), "root.pem", TestKey::RootKeyPem);

    // Create user
    setup::try_new_user(
        root_private_key_path.as_path(),
        root.path(),
        "Test User",
        "a.pub",
        TestKey::GoodKeyPub);

    // Create sequence
    let _ = setup::try_make_sequence(&root_private_key_path, "test_seq", "Dissonance.ogg");

    // Try to add permission to user
    setup::try_set_permission(
        &root.path(), 
        &root_private_key_path,
        true,
        "Test User",
        "EditSeq",
        Some("nonexistent".to_owned()),
        None::<u32>);

}

#[test]
#[should_panic(expected = "InvalidSequenceSection")]
fn fails_with_bad_target_editseqsec() {
    let root = setup::setup_init_cd();
    let root_private_key_path = common::make_key_file(root.path(), "root.pem", TestKey::RootKeyPem);

    // Create user
    setup::try_new_user(
        root_private_key_path.as_path(),
        root.path(),
        "Test User",
        "a.pub",
        TestKey::GoodKeyPub);

    // Create sequence
    let _ = setup::try_make_sequence(&root_private_key_path, "test_seq", "Dissonance.ogg");

    // Try to add permission to user
    setup::try_set_permission(
        &root.path(), 
        &root_private_key_path,
        true,
        "Test User",
        "EditSeqSec",
        Some("test_seq".to_owned()),
        Some(999));
}

#[test]
#[should_panic(expected = "No such file or directory")]
fn fails_with_bad_path_to_private_key() {
    let root = setup::setup_init_cd();
    let root_private_key_path = Path::new("undefined.pem");

    setup::try_new_user(
        root_private_key_path,
        root.path(),
        "Test User",
        "a.pub",
        TestKey::GoodKeyPub);

    setup::try_set_permission(
        &root.path(),
        &root_private_key_path,
        true,
        "Test User",
        "Administrate",
        None::<String>,
        None::<u32>);
}

#[test]
#[should_panic(expectd = "UserNotFound")]
fn fails_with_unused_private_key() {
    let root = setup::setup_init_cd();
    let root_private_key_path = common::make_key_file(root.path(), "root.pem", TestKey::GoodKey2Pem);
    
    // Create user
    setup::try_new_user(
        root_private_key_path.as_path(),
        root.path(),
        "Test User",
        "a.pub",
        TestKey::GoodKeyPub);

    setup::try_set_permission(
        &root.path(),
        &root_private_key_path,
        true,
        "Test User",
        "Administrate",
        None::<String>,
        None::<u32>);
}

#[test]
#[should_panic(expected = "UserNotFound")]
fn fails_with_nonexistent_username() {
    let root = setup::setup_init_cd();
    let root_private_key_path = common::make_key_file(root.path(), "root.pem", TestKey::RootKeyPem);

    setup::try_set_permission(
        &root.path(),
        &root_private_key_path,
        true,
        "Test User",
        "Administrate",
        None::<String>,
        None::<u32>);

}

#[test]
#[should_panic(expected = "UnauthorizedAction")]
fn fails_with_unauthorized_authority() {
    let root = setup::setup_init_cd();
    let root_private_key_path = common::make_key_file(root.path(), "root.pem", TestKey::RootKeyPem);

    // Create user
    setup::try_new_user(
        root_private_key_path.as_path(),
        root.path(),
        "Test User",
        "a.pub",
        TestKey::GoodKeyPub);
    let private_key_path = common::make_key_file(root.path(), "a.pem", TestKey::GoodKeyPem);

    setup::try_set_permission(
        &root.path(),
        &private_key_path,
        true,
        "root",
        "Administrate",
        None::<String>,
        None::<u32>);
}
