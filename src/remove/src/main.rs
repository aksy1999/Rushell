// extern crate colour;
extern crate colored; // not needed in Rust 2018
use colored::*;

fn main(){
	use std::env;
	let args: Vec<String> = env::args().collect();
	our_remove(&args);
}

pub fn our_remove(args: &Vec<String>) -> (){
    use std::io::ErrorKind;
	use std::path::Path;
	if args.len() == 1 {
		println!("{} {}", "Error: rm:".red().bold(), " no input arguments".red());
	}
	if args.len() == 2 {
		if args[1] == "-r"{
			println!("{} {}", "Error: rm:".red().bold(), "no input arguments".red());
		}
		else{
			println!("Removing file ...");
		}
	}
	else if args.len() > 2{
			println!("Removing files ...");

		let mut recursive = false;
		let mut start = 1;
		if args[1] == "-r" {
			recursive = true;
			start = 2;
		}
		let mut source = &args[0];
		for i in start..args.len(){
			source = &args[i];
			println!("  {}", &args[i]);
			// std::fs::remove_file(source).expect(" : Error: rm: File doesn't exist");
			if Path::new(source).exists(){
				if Path::new(source).is_file(){
					rm_file(source.to_string());
				}
				else if Path::new(source).is_dir() && recursive==false {
					println!("{} {}{}", "Error: rm:".red().bold(), source.red(), ": Path is a directory and not a file".red());
				}
		        else if Path::new(source).is_dir(){
					rm_dir(source.to_string());
				}
			}
			else{
				println!("{} {}{}", "Error: rm:".red().bold(), source.to_string().red(), ": Path to destination file does not exist".red());
			}
			// println!{""};
		}
	}
}

pub fn rm_file(source: String) -> () {

    use std::io::ErrorKind;
	match std::fs::remove_file(source.clone()) {
		Ok(file_in) => file_in,
		Err(error) => match error.kind() {
			ErrorKind::NotFound => {
				println!("{} {}{}", "Error: rm:".red().bold(), source.to_string().red(), ": Path does not exist".red());
				return
			}
			_other_error => {
				println!("{} {}{}", "Error: rm:".red().bold(), source.to_string().red(), ": Unexpected Error".red());
				return
			}
		},
	};
}

pub fn rm_dir(source: String) -> () {
    use std::io::ErrorKind;
	match std::fs::remove_dir_all(source.clone()) {
		Ok(file_in) => file_in,
		Err(error) => match error.kind() {
			ErrorKind::NotFound => {
				println!("{} {}{}", "Error: rm:".red().bold(), source.clone().to_string().red(), ": Path does not exist".red());
				return
			}
			_other_error => {
				println!("{} {}{}", "Error: rm:".red().bold(), source.to_string().red(), ": Unexpected Error".red());
				return
			}
		},
	};
}
