/// Executable for proton_cli

extern crate proton_cli;
extern crate git2;

use std::env;
use std::path::Path;

use git2::{Signature, Time};

fn main() {
    let root_arg = env::args().nth(1).unwrap();
    let root = Path::new(&root_arg);

    // TODO: Read a signature
    let time = Time::new(0, 0);
    let signature = Signature::new(
        "Proton Lights", "proton@teslaworks.net", &time).unwrap();

    match proton_cli::initialize_project(root, &signature) {
        Ok(_) => println!("Worked!"),
        Err(e) => println!("{:?}", e),
    }
}
