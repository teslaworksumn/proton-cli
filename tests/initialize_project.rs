extern crate proton_cli;
extern crate tempdir;
extern crate git2;
extern crate rustc_serialize;

use std::fs::File;
use std::path::Path;
use std::io::Read;

use git2::Repository;
use rustc_serialize::json;

use proton_cli::{Error, Project, initialize_project};
mod common;

#[test]
fn works_with_an_empty_root() {
    let root_dir = common::setup();

    let root = root_dir.path();
    initialize_project(root).expect("Initialization failed");

    assert_initialized(root);
}

#[test]
fn works_with_an_non_existent_root() {
    let root_dir = common::setup();

    let root = &root_dir.path().join("nonexistent");
    initialize_project(root).expect("Initialization failed");

    assert_initialized(root);
}

#[test]
#[should_panic(expected = "Initialization failed")]
fn fails_with_a_non_empty_directory() {
    let root_dir = common::setup();

    let root = root_dir.path();
    let _ = File::create(&root.join("unexpected")).expect("Making unexpected file failed");
    initialize_project(root).expect("Initialization failed");
}

fn assert_initialized(root: &Path) {
    // Assert that protonfile exists
    let protonfile_path = root.join(Path::new("Protonfile.json"));
    assert!(protonfile_path.is_file(), "protonfile must exist");

    // Check that protonfile has right content
    assert_eq!(Project::empty(), File::open(&protonfile_path)
        .and_then(|mut protonfile| {
            let mut content = "".to_owned();
            protonfile.read_to_string(&mut content).map(|_| content)
        })
        .map_err(Error::Io)
        .and_then(|content| json::decode(&content).map_err(Error::JsonDecode))
        .expect("Loading protonfile into Project failed"));

    // Open the git repo and master branch
    let repo = Repository::open(root).unwrap();
    let commit = repo.refname_to_id("refs/heads/master")
        .and_then(|oid| repo.find_commit(oid))
        .expect("Finding master failed");
    let tree = commit.tree().expect("Opening master tree failed");

    // Assert master is correct
    assert!(0 == commit.parents().count(), "master must have 0 parents");
    assert!(tree.get_name("Protonfile.json").is_some(), "master must have protonfile");
}