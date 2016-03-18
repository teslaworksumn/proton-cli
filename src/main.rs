/// Executable for proton_cli

extern crate proton_cli;
extern crate git2;

use std::env;
use std::path::Path;

fn main() {
    let root_arg = env::args().nth(1).unwrap();
    let root = Path::new(&root_arg);

    match proton_cli::initialize_project(root) {
        Ok(_) => println!("Worked!"),
        Err(e) => println!("{:?}", e),
    }
}
