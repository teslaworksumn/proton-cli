/// Executable for proton_cli

extern crate proton_cli;
extern crate git2;
extern crate rustc_serialize;
extern crate docopt;

use std::env;
use std::path::Path;
use docopt::Docopt;
use proton_cli::Error;


const USAGE: &'static str = "
Command-line interface for Proton

Usage:
  ./proton init <folder>
  ./proton new-user <name> <public-key>
  ./proton id-user <private-key>
  ./proton new-sequence <name> <music-file>
  ./proton (-h | --help)

Options:
  -h --help     Show this screen
";

#[derive(Debug, RustcDecodable)]
struct Args {
	cmd_init: bool,
	cmd_new_user: bool,
	cmd_id_user: bool,
	cmd_new_sequence: bool,
	arg_folder: Option<String>,
	arg_public_key: Option<String>,
	arg_private_key: Option<String>,
	arg_name: Option<String>,
	arg_music_file: Option<String>,
}

fn main() {
	let args: Args = Docopt::new(USAGE)
		.and_then(|d| d.decode())
		.unwrap_or_else(|e| e.exit());

	// Below unwrap()'s are safe within Docopt's usage rules

	let command: fn(Args) -> Result<(), Error> = match env::args().nth(1).unwrap().as_ref() {
		"init" => run_init,
		"new-user" => run_new_user,
		"id-user" => run_id_user,
		"new-sequence" => run_new_sequence,
		_ => panic!("Invalid first argument"),
	};

	let result = command(args);
	match result {
		Ok(_) => println!("Worked!"),
		Err(e) => println!("{:?}", e.to_string()),
	};

}

fn run_init(args: Args) -> Result<(), Error> {
	let root = args.arg_folder.unwrap();
	let root_path = Path::new(&root);
	proton_cli::initialize_project(&root_path)
}

fn run_new_user(args: Args) -> Result<(), Error> {
	let public_key = args.arg_public_key.unwrap();
	let public_key_path = Path::new(&public_key);
	let name = args.arg_name.unwrap();
	proton_cli::new_user(&public_key_path, &name)
}

fn run_id_user(args: Args) -> Result<(), Error> {
	let private_key = args.arg_private_key.unwrap();
	try!(proton_cli::id_user(&private_key)
		.map(|_| Ok(())))
}

fn run_new_sequence(args: Args) -> Result<(), Error> {
	let name = args.arg_name.unwrap();
	let music_file = args.arg_music_file.unwrap();
	let music_file_path = Path::new(&music_file);
	proton_cli::new_sequence(&name, &music_file_path)
}
