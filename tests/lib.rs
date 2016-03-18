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
        assert!(File::open(&protonfile_path)
            .and_then(|mut protonfile| {
                let mut content = "".to_owned();
                let read_result = protonfile.read_to_string(&mut content);
                //read_result.and(Ok(content))
                read_result.map(|_| content)
            })
            .map_err(Error::Io)
            .and_then(|content| json::decode(&content).map_err(Error::JsonDecode))
            .and_then(|project: Project| Ok(true))
            .expect("Protonfile check failed"));

        // Open a git repo
        // Check that there is exactly 1 commit
        // Check that it includes the protonfile
        // Check that it has the right signature
    }
}
