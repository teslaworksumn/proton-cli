/// Executable for proton_cli

extern crate proton_cli;

use std::env;
use proton_cli::init;
use proton_cli::project_types;

fn main() {
    let root = env::args().nth(1).unwrap();

    init::make_protonfile(&root);

    let initialized = init::make_project_folder(&root)
        .and_then(|_| init::make_repository(&root));

    match initialized {
        Ok(_) => println!("Worked!"),
        Err(e) => println!("{:?}", e),
    }
}