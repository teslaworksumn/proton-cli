extern crate proton_cli;

use std::env;
use std::path::PathBuf;


pub enum Key {
	GoodKeyPub,
	GoodKeyPem
}

fn get_tests_dir_path() -> PathBuf {
	let mut curr_dir = PathBuf::from(env::current_dir()
		.expect("Error getting current directory"));
	curr_dir.push("tests");
	curr_dir
}

pub fn get_key_file_path(key: Key) -> PathBuf {
	let key_path = match key {
		Key::GoodKeyPub => GOOD_KEY_PUB,
		Key::GoodKeyPem => GOOD_KEY_PEM,
	};

	let mut file_path = get_tests_dir_path();
	file_path.push(key_path);

	file_path
}

const GOOD_KEY_PUB: &'static str = "rsa_keys/good_key.pub";
const GOOD_KEY_PEM: &'static str = "rsa_keys/good_key.pem";
