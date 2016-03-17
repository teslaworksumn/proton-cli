/// Executable for proton_cli

extern crate proton_cli;

use std::env;
use std::path::Path;

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
    init::make_project_folder(root)
        .and_then(|_| init::make_protonfile(root))
        .and_then(|_| init::make_repository(root))
        .and_then(|repo| init::initial_commit(&repo))
}
