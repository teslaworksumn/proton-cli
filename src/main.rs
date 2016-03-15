/// Executable for proton_cli

extern crate proton_cli;

use std::env;

fn main() {
    let root = env::args().nth(1).unwrap();

    proton_cli::init::make_project_folder(&root);
}