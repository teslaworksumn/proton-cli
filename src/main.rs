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
use proton_cli::dao::{self, UserDao};


const USAGE: &'static str = "
Command-line interface for Proton

Usage:
  ./proton init <folder>
  ./proton new-user <admin-key> <name>
  ./proton remove-user <admin-key> <uid>
  ./proton new-sequence <admin-key> <name> <music-file>
  ./proton remove-sequence <admin-key> <seqid>
  ./proton delete-sequence <admin-key> <seqid>
  ./proton new-section <admin-key> <t_start> <t_end> <seqid> <fixid>..
  ./proton get-user-id <public-key>
  ./proton list-permissions <uid>
  ./proton set-permission <admin-key> (add | remove) <uid> Administrate
  ./proton set-permission <admin-key> (add | remove) <uid> EditSequence <target-sequence>
  ./proton set-permission <admin-key> (add | remove) <uid> EditSection <target-sequence> <target-section>
  ./proton test <seqid>
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
	arg_uid: Option<u32>,
	arg_seqid: Option<u32>,
	arg_fixid: Option<u32>,
	arg_music_file: Option<String>,
	arg_t_start: Option<u32>,
	arg_t_end: Option<u32>,
	arg_target_sequence: Option<u32>,
	arg_target_section: Option<u32>,
}

enum ProtonReturn {
	NoReturn,
	PublicKey(String),
}

fn main() {
	let args: Args = Docopt::new(USAGE)
		.and_then(|d| d.decode())
		.unwrap_or_else(|e| e.exit());

	// Below unwrap()'s are safe within Docopt's usage rules

	let command: fn(Args) -> Result<ProtonReturn, Error> = match env::args().nth(1).unwrap().as_ref() {
		"init" => run_init,
		"new-user" => run_new_user,
		"remove-user" => run_remove_user,
		"get_user_id" => run_get_user_id,
		"new-sequence" => run_new_sequence,
		"remove-sequence" => run_remove_sequence,
		"delete-sequence" => run_delete_sequence,
		"new-section" => run_new_section,
		"list-permissions" => run_list_permissions,
		"set-permission" => run_set_permission,
		"test" => run_test,
		_ => panic!("Invalid first argument"),
	};

	let result = command(args);
	match result {
		Ok(ret) => match ret {
			ProtonReturn::NoReturn => println!("Worked!"),
			ProtonReturn::PublicKey(s) => println!("{}", s),
		},
		Err(e) => println!("{:?}", e.to_string()),
	};

}

fn run_test(args: Args) -> Result<ProtonReturn, Error> {
	let chanid = args.arg_seqid.unwrap();
	let chan_dao = try!(dao::ChannelDaoPostgres::new());
	Ok(ProtonReturn::NoReturn)
}

fn run_init(args: Args) -> Result<ProtonReturn, Error> {
	let name = args.arg_name.unwrap();
	let root = args.arg_folder.unwrap();
	let root_path = Path::new(&root);
	let user_dao = try!(dao::UserDaoPostgres::new());
	let root_pub_key = try!(proton_cli::initialize_project(user_dao, &root_path, &name));
	Ok(ProtonReturn::PublicKey(root_pub_key))
}

fn run_new_user(args: Args) -> Result<ProtonReturn, Error> {
	let admin_key = args.arg_admin_key.unwrap();
	let admin_key_path = Path::new(&admin_key);
	let public_key = args.arg_public_key.unwrap();
	let public_key_path = Path::new(&public_key);
	let name = args.arg_name.unwrap();
	try!(proton_cli::new_user(&admin_key_path, &public_key_path, &name));
	Ok(ProtonReturn::NoReturn)
}

fn run_get_user_id(args: Args) -> Result<ProtonReturn, Error> {
	let public_key = args.arg_public_key.unwrap();
	let public_key_path = Path::new(&public_key);
	let user_dao = try!(dao::UserDaoPostgres::new());
	let uid = try!(user_dao.get_user_id(&public_key_path));
	println!("{:?}", uid);
	Ok(ProtonReturn::NoReturn)
}

fn run_remove_user(args: Args) -> Result<ProtonReturn, Error> {
	let admin_key = args.arg_admin_key.unwrap();
	let admin_key_path = Path::new(&admin_key);
	let uid = args.arg_uid.unwrap();
	try!(proton_cli::remove_user(&admin_key_path, uid));
	Ok(ProtonReturn::NoReturn)
}

fn run_new_section(args: Args) -> Result<ProtonReturn, Error> {
	Err(Error::TodoErr)
}

fn run_new_sequence(args: Args) -> Result<ProtonReturn, Error> {
	let admin_key = args.arg_admin_key.unwrap();
	let admin_key_path = Path::new(&admin_key);
	let name = args.arg_name.unwrap();
	let music_file = args.arg_music_file.unwrap();
	let music_file_path = Path::new(&music_file);
	try!(proton_cli::new_sequence(&admin_key_path, &name, &music_file_path, None::<u32>));
	Ok(ProtonReturn::NoReturn)
}

fn run_remove_sequence(args: Args) -> Result<ProtonReturn, Error> {
	let admin_key = args.arg_admin_key.unwrap();
	let admin_key_path = Path::new(&admin_key);
	let seqid = args.arg_seqid.unwrap();
	try!(proton_cli::remove_sequence(&admin_key_path, seqid));
	Ok(ProtonReturn::NoReturn)
}

fn run_delete_sequence(args: Args) -> Result<ProtonReturn, Error> {
	let admin_key = args.arg_admin_key.unwrap();
	let admin_key_path = Path::new(&admin_key);
	let seqid = args.arg_seqid.unwrap();
	let sequence_dao = try!(dao::SequenceDaoPostgres::new());
	try!(proton_cli::delete_sequence(sequence_dao, &admin_key_path, seqid));
	Ok(ProtonReturn::NoReturn)
}

fn run_list_permissions(args: Args) -> Result<ProtonReturn, Error> {
	let uid = args.arg_uid.unwrap();
	let perm_dao = try!(dao::PermissionDaoPostgres::new());
	let permissions = try!(
		proton_cli::get_permissions::<String, dao::PermissionDaoPostgres>(perm_dao, uid));
	println!("{}", json::as_pretty_json(&permissions));
	Ok(ProtonReturn::NoReturn)
}

fn run_set_permission(args: Args) -> Result<ProtonReturn, Error> {
	let admin_key = args.arg_admin_key.unwrap();
	let admin_key_path = Path::new(&admin_key);
	let added = env::args().nth(3).unwrap() == "add";
	let uid = args.arg_uid.unwrap();
	let permission_name = env::args().nth(5).unwrap();
	let target_sequence = args.arg_target_sequence;
	let target_section = args.arg_target_section;

	try!(proton_cli::set_permission(
		&admin_key_path,
		added,
		uid,
		&permission_name,
		target_sequence,
		target_section));
	Ok(ProtonReturn::NoReturn)
}
