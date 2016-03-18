#![feature(plugin)]
#![plugin(stainless)]

extern crate proton_cli;
extern crate tempdir;
extern crate git2;
extern crate rustc_serialize;

pub use std::{env, fs};
pub use std::fs::File;
pub use std::path::{Path, PathBuf};
pub use std::io::Read;

pub use git2::{Repository, Signature, Time};
pub use tempdir::TempDir;
pub use rustc_serialize::json;

pub use proton_cli::{Error, Project, initialize_project};

describe! initialize_project {
    before_each {
        let signature = Signature::now("tester", "t@example.com").unwrap();
        let root_dir = TempDir::new("proton_cli_tests").unwrap();
    }

    it "works with an empty directory" {
        let root = root_dir.path();
        initialize_project(root, &signature).expect("Initialization failed");
    }

    after_each {
        // Assert that exactly Protonfile.json and .git exist.
        let protonfile_path = root.join(Path::new("Protonfile.json"));
        let git_path = root.join(Path::new(".git"));
        assert!(protonfile_path.is_file());
        assert!(git_path.is_dir());
        assert_eq!(2, fs::read_dir(root).unwrap().count());

        // Check that protonfile has right content
        assert_eq!(Project::empty(), File::open(&protonfile_path)
            .and_then(|mut protonfile| {
                let mut content = "".to_owned();
                protonfile.read_to_string(&mut content).map(|_| content)
            })
            .map_err(Error::Io)
            .and_then(|content| json::decode(&content).map_err(Error::JsonDecode))
            .expect("Loading protonfile into Project failed"));

        // Open a git repo
        let repo = Repository::open(root).unwrap();
        let master_head = repo.refname_to_id("refs/heads/master")
            .and_then(|oid| repo.find_commit(oid))
            .expect("Finding master failed");
        let master_tree = master_head.tree().expect("Opening master tree failed");

        // Check that master has exactly 1 commit
        assert_eq!(0, master_head.parents().count());;

        // Check that it includes the protonfile
        

        // Check that it has the right signature
    }
}
