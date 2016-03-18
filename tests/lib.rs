#![feature(plugin)]
#![plugin(stainless)]

extern crate proton_cli;
extern crate tempdir;
extern crate git2;

pub use std::{env, fs};
pub use std::path::{Path, PathBuf};

pub use git2::{Repository, Signature, Time};
pub use tempdir::TempDir;

pub use proton_cli::{Error, initialize_project};

describe! initialize_project {
    before_each {
        let signature = Signature::now(
            "Proton Lights", "proton@teslaworks.net").unwrap();
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

        // Open a git repo
        // Check that there is exactly 1 commit
        // Check that it includes the protonfile
        // Check that it has the right signature
    }
}
