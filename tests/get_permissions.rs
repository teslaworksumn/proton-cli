extern crate proton_cli;

mod common;

use std::path::Path;

use common::setup;
use common::rsa_keys::TestKey;
use proton_cli::project_types::PermissionEnum;


#[test]
fn works_with_valid_key_no_permissions() {
    let root = setup::setup_init_cd();
    let root_key_path = common::make_key_file(&root.path(), "root.pem", TestKey::RootKeyPem);
    let user_key_path = common::make_key_file(&root.path(), "a.pem", TestKey::GoodKeyPem);
    let name = "UserA";

    setup::try_new_user(
        &root_key_path.as_path(),
        root.path(),
        &name,
        "a.pub",
        TestKey::GoodKeyPub);

    let permissions = proton_cli::get_permissions(&user_key_path)
        .expect("Error getting permissions");

    assert_eq!(permissions.len(), 0);
}

#[test]
fn works_with_valid_key_one_permission() {
    let root = setup::setup_init_cd();
    let root_key_path = common::make_key_file(&root.path(), "root.pem", TestKey::RootKeyPem);
    let user_key_path = common::make_key_file(&root.path(), "a.pem", TestKey::GoodKeyPem);
    let name = "UserA";

    setup::try_new_user(
        &root_key_path.as_path(),
        root.path(),
        &name,
        "a.pub",
        TestKey::GoodKeyPub);

    setup::try_make_sequence(
        &root_key_path.as_path(),
        "asdf",
        "Dissonance.ogg"
    );

    setup::try_set_permission(
        &root.path(),
        &root_key_path,
        true,
        &name,
        PermissionEnum::EditSeq,
        Some("asdf".to_owned()));
    
    let permissions = proton_cli::get_permissions(&user_key_path)
        .expect("Error getting permissions");

    assert_eq!(permissions.len(), 1);
    assert_eq!(&permissions[0].which, &PermissionEnum::EditSeq);
    assert!(&permissions[0].target.is_some());
    assert_eq!(permissions[0].target, Some("asdf".to_owned()));
}

#[test]
fn works_with_valid_key_all_permissions() {
    let root = setup::setup_init_cd();
    let root_key_path = common::make_key_file(&root.path(), "root.pem", TestKey::RootKeyPem);
    let user_key_path = common::make_key_file(&root.path(), "a.pem", TestKey::GoodKeyPem);
    let name = "UserA";

    setup::try_new_user(
        &root_key_path.as_path(),
        root.path(),
        &name,
        "a.pub",
        TestKey::GoodKeyPub);

    setup::try_make_sequence(
        &root_key_path.as_path(),
        "asdf",
        "Dissonance.ogg"
    );

    setup::try_make_sequence(
        &root_key_path.as_path(),
        "ghjk",
        "GlorytotheBells.ogg"
    );

    setup::try_set_permission(
        &root.path(),
        &root_key_path,
        true,
        &name,
        PermissionEnum::Administrate,
        None::<String>);

    setup::try_set_permission(
        &root.path(),
        &root_key_path,
        true,
        &name,
        PermissionEnum::EditSeq,
        Some("asdf".to_owned()));

    setup::try_set_permission(
        &root.path(),
        &root_key_path,
        true,
        &name,
        PermissionEnum::EditSeqSec,
        Some("ghjk,1".to_owned()));

    let permissions = proton_cli::get_permissions(&user_key_path)
        .expect("Error getting permissions");

    assert_eq!(permissions.len(), 3);
    assert_eq!(&permissions[0].which, &PermissionEnum::Administrate);
    assert!(&permissions[0].target.is_none());
    assert_eq!(&permissions[1].which, &PermissionEnum::EditSeq);
    assert!(&permissions[1].target.is_some());
    assert_eq!(permissions[1].target, Some("asdf".to_owned()));
    assert_eq!(&permissions[2].which, &PermissionEnum::EditSeqSec);
    assert!(&permissions[2].target.is_some());
    assert_eq!(permissions[2].target, Some("ghjk,1".to_owned()));
}

#[test]
#[should_panic(expected = "Error getting permissions: Io")]
fn fails_with_invalid_key_path() {
    let root = setup::setup_init_cd();
    let root_key_path = common::make_key_file(&root.path(), "root.pem", TestKey::RootKeyPem);
    let user_key_path = Path::new("invalid");
    let name = "UserA";

    setup::try_new_user(
        &root_key_path.as_path(),
        root.path(),
        &name,
        "a.pub",
        TestKey::GoodKeyPub);

    let _ = proton_cli::get_permissions(&user_key_path)
        .expect("Error getting permissions");
}

#[test]
#[should_panic(expected = "Error getting permissions: Ssl")]
fn fails_with_invalid_key() {
    let root = setup::setup_init_cd();
    let root_key_path = common::make_key_file(&root.path(), "root.pem", TestKey::RootKeyPem);
    let user_key_path = common::make_key_file(&root.path(), "a.pub", TestKey::GoodKeyPub);
    let name = "UserA";

    setup::try_new_user(
        &root_key_path.as_path(),
        root.path(),
        &name,
        "aa.pub",
        TestKey::GoodKeyPub);

    let _ = proton_cli::get_permissions(&user_key_path)
        .expect("Error getting permissions");
}
