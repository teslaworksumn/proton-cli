/// Executable for proton_cli

extern crate proton_cli;
extern crate git2;

use std::env;
use std::path::Path;

use git2::{Signature, Time};

use proton_cli::{init, Error};

fn main() {
    let root_arg = env::args().nth(1).unwrap();
    let root = Path::new(&root_arg);

    match init(root) {
        Ok(_) => println!("Worked!"),
        Err(e) => println!("{:?}", e),
    }
}

fn init(root: &Path) -> Result<(), Error> {
    // TODO: Read a signature
    let time = Time::new(0, 0);
    let signature = Signature::new(
        "Proton Lights", "proton@teslaworks.net", &time).unwrap();

    init::make_project_folder(root)
        .and_then(|_| init::make_protonfile(root))
        .and_then(|_| init::make_repository(root))
        .and_then(|repo| init::initial_commit(&repo, &signature))
        .map(|_| ())
}
