use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;

fn main() {
	let path = Path::new(".");
	println!("Image Name\t\tPID");
	for entry in fs::read_dir(path).unwrap() {
		//let entry_path = entry.to_str();
		println!("{:?}", entry.unwrap().path());
		//if p.chars().all(char::is_numeric){
		//}
	}
}
