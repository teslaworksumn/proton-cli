/// Executable for proton_cli

extern crate proton_cli;

use std::env;
use proton_cli::init;

fn main() {
    let root = env::args().nth(1).unwrap();

    let initialized = init::make_project_folder(&root)
        .and_then(|_| init::make_repository(&root));

    match initialized {
        Ok(_) => println!("Worked!"),
        Err(e) => println!("{:?}", e),
    }
}