extern crate proton_cli;

mod dao;
mod common;

use common::TestKey;

use proton_cli::error::Error;


#[test]
// This test is pretty simple, but it checks that if the user_dao
// doesn't throw an error, then its value is what is returned by get_userid
fn proper_id_returned_with_matching_public_key() {
    let pub_key_path = common::get_key_file_path(TestKey::GoodKeyPub);
    let mut user_dao = dao::UserDaoTesting::new();
    user_dao.get_user_id_fn = Box::new(|_| { Ok(1) });
    let expected = 1;
    let actual = proton_cli::get_user_id(user_dao, pub_key_path).expect("Error getting user id");
    assert_eq!(expected, actual);
}

#[test]
#[should_panic(expected = "InvalidPublicKey")]
// Indirectly checks if invalid key given
fn fails_if_private_key_given() {
    let priv_key_path = common::get_key_file_path(TestKey::GoodKeyPem);
    let mut user_dao = dao::UserDaoTesting::new();
    user_dao.get_user_id_fn = Box::new(|_| { Ok(1) });
    let _ = proton_cli::get_user_id(user_dao, priv_key_path).expect("Error getting user id");
}

#[test]
#[should_panic(expected = "PublicKeyNotFound")]
fn fails_if_public_key_given_doesnt_match() {
    let pub_key_path = common::get_key_file_path(TestKey::GoodKeyPub);
    let mut user_dao = dao::UserDaoTesting::new();
    user_dao.get_user_id_fn = Box::new(|pub_key| {
        Err(Error::PublicKeyNotFound(pub_key))
    });
    let _ = proton_cli::get_user_id(user_dao, pub_key_path).expect("Error getting user id");    
}

#[test]
#[should_panic(expected = "FileNotFound")]
fn fails_if_path_to_key_nonexistent() {
    let priv_key_path = "bad_path";
    let mut user_dao = dao::UserDaoTesting::new();
    user_dao.get_user_id_fn = Box::new(|_| { Ok(1) });
    let _ = proton_cli::get_user_id(user_dao, priv_key_path).expect("Error getting user id");   
}
