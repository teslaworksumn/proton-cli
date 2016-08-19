/// Executable for proton_cli
extern crate rustc_serialize;
extern crate proton_cli;
extern crate docopt;

use std::env;
use std::path::Path;
use rustc_serialize::json;
use docopt::Docopt;

use proton_cli::error::Error;
use proton_cli::utils;


const USAGE: &'static str = "
Command-line interface for Proton

Usage:
  ./proton init <folder> <root-public-key>
  ./proton new-user <admin-key> <name> <public-key>
  ./proton remove-user <admin-key> <name>
  ./proton new-sequence <admin-key> <name> <music-file>
  ./proton remove-sequence <admin-key> <name>
  ./proton resection-sequence <admin-key> <name> <num-sections>
  ./proton id-user <private-key>
  ./proton list-permissions <private-key>
  ./proton set-permission <admin-key> (add | remove) <name> Administrate
  ./proton set-permission <admin-key> (add | remove) <name> EditSeq <target-sequence>
  ./proton set-permission <admin-key> (add | remove) <name> EditSeqSec <target-section>
  ./proton (-h | --help)

Options:
  -h --help     Show this screen
";

#[derive(Debug, RustcDecodable)]
struct Args {
	arg_folder: Option<String>,
	arg_root_public_key: Option<String>,
	arg_public_key: Option<String>,
	arg_private_key: Option<String>,
	arg_admin_key: Option<String>,
	arg_name: Option<String>,
	arg_music_file: Option<String>,
	arg_target_sequence: Option<String>,
	arg_target_section: Option<u32>,
	arg_num_sections: Option<u32>,
}

fn main() {
	let args: Args = Docopt::new(USAGE)
		.and_then(|d| d.decode())
		.unwrap_or_else(|e| e.exit());

	// Below unwrap()'s are safe within Docopt's usage rules

	let command: fn(Args) -> Result<(), Error> = match env::args().nth(1).unwrap().as_ref() {
		"init" => run_init,
		"new-user" => run_new_user,
		"remove-user" => run_remove_user,
		"id-user" => run_id_user,
		"new-sequence" => run_new_sequence,
		"remove-sequence" => run_remove_sequence,
		"resection-sequence" => run_resection_sequence,
		"list-permissions" => run_list_permissions,
		"set-permission" => run_set_permission,
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

	let root_pub_key_path = args.arg_root_public_key.unwrap();
	let root_pub_key = try!(utils::file_as_string(&root_pub_key_path));
	
	proton_cli::initialize_project(&root_path, &root_pub_key)
}

fn run_new_user(args: Args) -> Result<(), Error> {
	let admin_key = args.arg_admin_key.unwrap();
	let admin_key_path = Path::new(&admin_key);
	let public_key = args.arg_public_key.unwrap();
	let public_key_path = Path::new(&public_key);
	let name = args.arg_name.unwrap();
	proton_cli::new_user(&admin_key_path, &public_key_path, &name)
}

fn run_id_user(args: Args) -> Result<(), Error> {
	let private_key = args.arg_private_key.unwrap();
	let user = try!(proton_cli::id_user(&private_key));
	Ok(println!("{:?}", user))
}

fn run_remove_user(args: Args) -> Result<(), Error> {
	let admin_key = args.arg_admin_key.unwrap();
	let admin_key_path = Path::new(&admin_key);
	let name = args.arg_name.unwrap();
	proton_cli::remove_user(&admin_key_path, &name)
}

fn run_new_sequence(args: Args) -> Result<(), Error> {
	let admin_key = args.arg_admin_key.unwrap();
	let admin_key_path = Path::new(&admin_key);
	let name = args.arg_name.unwrap();
	let music_file = args.arg_music_file.unwrap();
	let music_file_path = Path::new(&music_file);
	proton_cli::new_sequence(&admin_key_path, &name, &music_file_path)
}

fn run_remove_sequence(args: Args) -> Result<(), Error> {
	let admin_key = args.arg_admin_key.unwrap();
	let admin_key_path = Path::new(&admin_key);
	let name = args.arg_name.unwrap();
	proton_cli::remove_sequence(&admin_key_path, &name)
}

fn run_resection_sequence(args: Args) -> Result<(), Error> {
	let admin_key = args.arg_admin_key.unwrap();
	let admin_key_path = Path::new(&admin_key);
	let name = args.arg_name.unwrap();
	let num_sections = args.arg_num_sections.unwrap();
	proton_cli::resection_sequence(&admin_key_path, &name, num_sections)
}

fn run_list_permissions(args: Args) -> Result<(), Error> {
	let private_key = args.arg_private_key;
	proton_cli::get_permissions(&private_key.unwrap())
		.map(|p| println!("{}", json::as_pretty_json(&p)))
}

fn run_set_permission(args: Args) -> Result<(), Error> {
	let admin_key = args.arg_admin_key.unwrap();
	let auth_user = try!(proton_cli::id_user(&admin_key));

	let added = env::args().nth(3).unwrap() == "add";
	let username = args.arg_name.unwrap();
	let permission_name = env::args().nth(5).unwrap();
	let target_sequence = args.arg_target_sequence;
	let target_section = args.arg_target_section;

	proton_cli::set_permission(
		&auth_user,
		added,
		&username,
		&permission_name,
		target_sequence,
		target_section)
}
