extern crate proton_cli;
extern crate tempdir;
extern crate git2;

mod common;

use std::path::{Path, PathBuf};
use std::fs;
use tempdir::TempDir;

use common::setup;
use common::rsa_keys::TestKey;
use common::sequence_sections::{self, TestSeqSec};
use proton_cli::project_types::Sequence;


fn setup_resection(user_key: TestKey, has_perm: bool) -> (TempDir, PathBuf, Sequence) {
    let root = setup::setup_init_cd();
    let root_key_path = common::make_key_file(&root.path(), "root.pem", TestKey::RootKeyPem);
    let user_key_path = common::make_key_file(&root.path(), "usera.pem", user_key);
    let name = "SequenceA";
    let sequence = setup::try_make_sequence(&root_key_path.as_path(), &name, "test_1sec.ogg");
    assert_eq!(sequence.num_sections, 1);    
    let _ = sequence.set_section_data(1, sequence_sections::get_test_seq_sec(TestSeqSec::Good1of1))
        .expect("Error setting sequence data");
    assert_section_correct(&sequence, 1, TestSeqSec::Good1of1);

    setup::try_new_user(
        &root_key_path.as_path(),
        &root.path(),
        &"UserA",
        &"usera.pub",
        TestKey::GoodKeyPub);

    if has_perm {
        setup::try_set_permission(
            &root.path(),
            &root_key_path,
            true,
            "UserA",
            "EditSeq",
            Some(name.to_owned()),
            None::<u32>);
    }

    (root, user_key_path, sequence)
}

/// Get a section from the sequence and make sure its data is the same as the test_section given
fn assert_section_correct(sequence: &Sequence, section_idx: u32, test_section: TestSeqSec) {
    let section = sequence.get_section(section_idx).expect("Error retrieving sequence section");
    println!("index {}: {:?}", section_idx, section.data);
    assert_eq!(section.data, sequence_sections::get_test_seq_sec(test_section));
}

/// Gets the sequence named SequenceA, created in setup_resection
fn get_sequence() -> Sequence {
    let project = proton_cli::utils::read_protonfile(None::<&Path>)
        .expect("Error reading protonfile");
    project.find_sequence_by_name(&"SequenceA")
        .expect("Sequence somehow removed during resection")
        .to_owned()
}

#[test]
#[allow(unused_variables)]
// root reference must be kept to keep temp directory in scope, but is never used
fn works_with_valid_inputs() {
    let (root, user_key_path, sequence) = setup_resection(TestKey::GoodKeyPem, true);
    let name = &sequence.name;

    // Try different resections
    let _ = proton_cli::resection_sequence(&user_key_path.as_path(), name, 2)
        .expect("Error resectioning sequence");
    let seq_1_to_2 = get_sequence();
    assert_eq!(seq_1_to_2.num_sections, 2);
    assert_section_correct(&seq_1_to_2, 1, TestSeqSec::Good1of2);
    assert_section_correct(&seq_1_to_2, 2, TestSeqSec::Good2of2);
    common::assert_repo_no_modified_files(&root.path());

    let _ = proton_cli::resection_sequence(&user_key_path.as_path(), name, 3)
        .expect("Error resectioning sequence");
    let seq_2_to_3 = get_sequence();
    assert_eq!(seq_2_to_3.num_sections, 3);
    assert_section_correct(&seq_2_to_3, 1, TestSeqSec::Good1of3);
    assert_section_correct(&seq_2_to_3, 2, TestSeqSec::Good2of3);
    assert_section_correct(&seq_2_to_3, 3, TestSeqSec::Good3of3);
    common::assert_repo_no_modified_files(&root.path());

    let _ = proton_cli::resection_sequence(&user_key_path.as_path(), name, 1)
        .expect("Error resectioning sequence");
    let seq_3_to_1 = get_sequence();
    assert_eq!(seq_3_to_1.num_sections, 1);
    assert_section_correct(&seq_3_to_1, 1, TestSeqSec::Good1of1);
    common::assert_repo_no_modified_files(&root.path());

    let _ = proton_cli::resection_sequence(&user_key_path.as_path(), name, 3)
        .expect("Error resectioning sequence");
    let seq_1_to_3 = get_sequence();
    assert_eq!(seq_1_to_3.num_sections, 3);
    assert_section_correct(&seq_1_to_3, 1, TestSeqSec::Good1of3);
    assert_section_correct(&seq_1_to_3, 2, TestSeqSec::Good2of3);
    assert_section_correct(&seq_1_to_3, 3, TestSeqSec::Good3of3);
    common::assert_repo_no_modified_files(&root.path());

    let _ = proton_cli::resection_sequence(&user_key_path.as_path(), name, 2)
        .expect("Error resectioning sequence");
    let seq_3_to_2 = get_sequence();
    assert_eq!(seq_3_to_2.num_sections, 2);
    assert_section_correct(&seq_3_to_2, 1, TestSeqSec::Good1of2);
    assert_section_correct(&seq_3_to_2, 2, TestSeqSec::Good2of2);
    common::assert_repo_no_modified_files(&root.path());
}

#[test]
#[allow(unused_variables)]
// root reference must be kept to keep temp directory in scope, but is never used
#[should_panic(expected = "Error resectioning sequence: Ssl")]
fn fails_with_bad_user_key() {
    let (root, user_key_path, sequence) = setup_resection(TestKey::GoodKey2Pub, true);
    let _ = proton_cli::resection_sequence(
        &user_key_path.as_path(),
        &sequence.name,
        2).expect("Error resectioning sequence");
}

#[test]
#[allow(unused_variables)]
// root reference must be kept to keep temp directory in scope, but is never used
#[should_panic(expected = "Error resectioning sequence: Io")]
fn fails_with_nonexistent_user_key() {
    let (root, _, sequence) = setup_resection(TestKey::GoodKeyPem, true);
    let bad_key_path = PathBuf::from("nonexistent");
    let _ = proton_cli::resection_sequence(
        &bad_key_path.as_path(),
        &sequence.name,
        2).expect("Error resectioning sequence");
}

#[test]
#[allow(unused_variables)]
// root reference must be kept to keep temp directory in scope, but is never used
#[should_panic(expected = "Error resectioning sequence: UnauthorizedAction")]
fn fails_with_unprivileged_user_key() {
    let (root, user_key_path, sequence) = setup_resection(TestKey::GoodKeyPem, false);
    let _ = proton_cli::resection_sequence(
        &user_key_path.as_path(),
        &sequence.name,
        2).expect("Error resectioning sequence");
}

#[test]
#[allow(unused_variables)]
// root reference must be kept to keep temp directory in scope, but is never used
#[should_panic(expected = "Error resectioning sequence: InvalidPermissionTarget")]
fn fails_with_nonexistent_sequence_name() {
    let (root, user_key_path, _) = setup_resection(TestKey::GoodKeyPem, false);
    let bad_name = &"notasequencename";
    let _ = proton_cli::resection_sequence(&user_key_path.as_path(), bad_name, 2)
        .expect("Error resectioning sequence");
}

#[test]
#[allow(unused_variables)]
// root reference must be kept to keep temp directory in scope, but is never used
#[should_panic(expected = "Error resectioning sequence: InvalidPermissionTarget")]
fn fails_with_invalid_sequence_name() {
    let (root, user_key_path, _) = setup_resection(TestKey::GoodKeyPem, false);
    let invalid_name = &"Not a valid seq name! ;)";
    let _ = proton_cli::resection_sequence(&user_key_path.as_path(), invalid_name, 2)
        .expect("Error resectioning sequence");
}

#[test]
#[allow(unused_variables)]
// root reference must be kept to keep temp directory in scope, but is never used
#[should_panic(expected = "Error resectioning sequence: SequenceSectionNotFound")]
fn fails_if_section_file_deleted_after_creation() {
    // Create sequence, then delete sequence/sequence_sec1
    // Check vector of paths on sequence load, warn/remove dead paths
    let (root, user_key_path, sequence) = setup_resection(TestKey::GoodKeyPem, true);
    let section = sequence.get_section(1).expect("Error getting sequence section");
    let _ = fs::remove_file(&section.path).expect("Error removing seq sec file");
    let _ = proton_cli::resection_sequence(
        &user_key_path.as_path(),
        &sequence.name,
        2).expect("Error resectioning sequence");
}

#[test]
#[allow(unused_variables)]
// root reference must be kept to keep temp directory in scope, but is never used
#[should_panic(expected = "Error resectioning sequence: InvalidSequenceSection")]
fn fails_0_sections() {
    let (root, user_key_path, sequence) = setup_resection(TestKey::GoodKeyPem, true);
    let _ = proton_cli::resection_sequence(
        &user_key_path.as_path(),
        &sequence.name,
        0).expect("Error resectioning sequence");
}
