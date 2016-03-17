/// Executable for proton_cli

extern crate proton_cli;

use std::env;
use proton_cli::{init, Error};

fn main() {
    let root = env::args().nth(1).unwrap();

    match init(&root) {
        Ok(_) => println!("Worked!"),
        Err(e) => println!("{:?}", e),
    }
}

fn init(root: &str) -> Result<(), Error> {
    init::make_project_folder(root)
        .and_then(|_| init::make_protonfile(root))
        .and_then(|_| init::make_repository(root))
        .and_then(|repo| init::initial_commit(&repo))
}