use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;

fn main() {
	let path = Path::new(".");
	println!("Image Name\t\tPID");
	for entry in fs::read_dir(path) {
		println!("{}", entry);
		//if p.chars().all(char::is_numeric){
		//}
	}
}