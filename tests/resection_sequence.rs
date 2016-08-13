extern crate proton_cli;
extern crate tempdir;
extern crate git2;

mod common;

use std::path::PathBuf;

use common::setup;
use common::rsa_keys::TestKey;
use common::sequence_sections::{self, TestSeqSec};
use proton_cli::project_types::{PermissionEnum};


fn setup_resection<'a>() -> (PathBuf, &'a str) {
    let root = setup::setup_init_cd();
    let root_key_path = common::make_key_file(&root.path(), "root.pem", TestKey::RootKeyPem);
    let user_key_path = common::make_key_file(&root.path(), "usera.pem", TestKey::GoodKeyPem);
    let name = "SequenceA";
    let sequence = setup::try_make_sequence(&root_key_path.as_path(), &name, "test_1sec.ogg");
    assert_eq!(sequence.num_sections, 1);
    let mut seq_sec_1 = sequence.get_section(1).expect("Error retrieving sequence section");
    seq_sec_1.data = sequence_sections::get_test_seq_sec(TestSeqSec::Good1of1);
    assert_eq!(seq_sec_1.data, sequence_sections::get_test_seq_sec(TestSeqSec::Good1of1));

    setup::try_new_user(
        &root_key_path.as_path(),
        &root.path(),
        &"UserA",
        &"usera.pub",
        TestKey::GoodKeyPub);

    setup::try_set_permission(
        &root.path(),
        &root_key_path,
        true,
        "UserA",
        PermissionEnum::EditSeq,
        Some(name.to_owned()));

    (user_key_path, &name)
}

#[test]
fn works_1_to_2() {
    let (user_key_path, name) = setup_resection();

    let seq_1_to_2 = proton_cli::resection_sequence(&user_key_path.as_path(), name, 2)
        .expect("Error resectioning sequence");
    assert_eq!(seq_1_to_2.num_sections, 2);
    let seq_sec_1_of_2 = seq_1_to_2.get_section(1).expect("Error retrieving sequence section");
    let seq_sec_2_of_2 = seq_1_to_2.get_section(2).expect("Error retrieving sequence section");
    assert_eq!(seq_sec_1_of_2.data, sequence_sections::get_test_seq_sec(TestSeqSec::Good1of2));
    assert_eq!(seq_sec_2_of_2.data, sequence_sections::get_test_seq_sec(TestSeqSec::Good2of2));
}

#[test]
fn works_1_to_3() {
    let (user_key_path, name) = setup_resection();
    let seq_1_to_3 = proton_cli::resection_sequence(&user_key_path.as_path(), &name, 3)
        .expect("Error resectioning sequence");
    assert_eq!(seq_1_to_3.num_sections, 3);
    let seq_sec_1_of_3 = seq_1_to_3.get_section(1).expect("Error retrieving sequence section");
    let seq_sec_2_of_3 = seq_1_to_3.get_section(2).expect("Error retrieving sequence section");
    let seq_sec_3_of_3 = seq_1_to_3.get_section(3).expect("Error retrieving sequence section");
    assert_eq!(seq_sec_1_of_3.data, sequence_sections::get_test_seq_sec(TestSeqSec::Good1of3));
    assert_eq!(seq_sec_2_of_3.data, sequence_sections::get_test_seq_sec(TestSeqSec::Good2of3));
    assert_eq!(seq_sec_3_of_3.data, sequence_sections::get_test_seq_sec(TestSeqSec::Good3of3));
}

#[test]
fn works_2_to_3() {
    let (user_key_path, name) = setup_resection();
    // Initially split into 2 before trying to split further
    let _ = proton_cli::resection_sequence(&user_key_path.as_path(), &name, 2)
        .expect("Error doing initial resequence");
    let seq_2_to_3 = proton_cli::resection_sequence(&user_key_path.as_path(), &name, 3)
        .expect("Error resectioning sequence");
    assert_eq!(seq_2_to_3.num_sections, 3);
    let seq_sec_1_of_3 = seq_2_to_3.get_section(1).expect("Error retrieving sequence section");
    let seq_sec_2_of_3 = seq_2_to_3.get_section(2).expect("Error retrieving sequence section");
    let seq_sec_3_of_3 = seq_2_to_3.get_section(3).expect("Error retrieving sequence section");
    assert_eq!(seq_sec_1_of_3.data, sequence_sections::get_test_seq_sec(TestSeqSec::Good1of3));
    assert_eq!(seq_sec_2_of_3.data, sequence_sections::get_test_seq_sec(TestSeqSec::Good2of3));
    assert_eq!(seq_sec_3_of_3.data, sequence_sections::get_test_seq_sec(TestSeqSec::Good3of3));
}

#[test]
fn works_3_to_2() {
    let (user_key_path, name) = setup_resection();
    // Initially split into 3 before trying to merge back
    let _ = proton_cli::resection_sequence(&user_key_path.as_path(), &name, 3)
        .expect("Error doing initial resequence");
    let seq_3_to_2 = proton_cli::resection_sequence(&user_key_path.as_path(), &name, 2)
        .expect("Error resectioning sequence");
    assert_eq!(seq_3_to_2.num_sections, 2);
    let seq_sec_1_of_2 = seq_3_to_2.get_section(1).expect("Error retrieving sequence section");
    let seq_sec_2_of_2 = seq_3_to_2.get_section(2).expect("Error retrieving sequence section");
    assert_eq!(seq_sec_1_of_2.data, sequence_sections::get_test_seq_sec(TestSeqSec::Good1of2));
    assert_eq!(seq_sec_2_of_2.data, sequence_sections::get_test_seq_sec(TestSeqSec::Good2of2));
}

#[test]
fn works_3_to_1() {
    let (user_key_path, name) = setup_resection();
    // Initially split into 3 before trying to merge back
    let _ = proton_cli::resection_sequence(&user_key_path.as_path(), &name, 3)
        .expect("Error doing initial resequence");
    let seq_3_to_1 = proton_cli::resection_sequence(&user_key_path.as_path(), &name, 1)
        .expect("Error resectioning sequence");
    assert_eq!(seq_3_to_1.num_sections, 1);
    let seq_sec_1_of_1 = seq_3_to_1.get_section(1).expect("Error retrieving sequence section");
    assert_eq!(seq_sec_1_of_1.data, sequence_sections::get_test_seq_sec(TestSeqSec::Good1of1));
}
