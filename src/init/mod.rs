//! This module initializes a project.

use std::fs;

/// Creates a folder. The folder must not exist or must be empty.
///
/// Impure.
fn make_project_folder(root: &str) {
    // Make the folder - ignore error.
    fs::create_dir(root);

    // Check that the folder is empty.
    if 0 != fs::read_dir(root).unwrap().count() {
        panic!("FATAL: The folder at {} isn't empty!");
    }
}