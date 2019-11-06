extern crate colored; // not needed in Rust 2018

use colored::*;


fn main(){
	use std::env;
	let args: Vec<String> = env::args().collect();
	our_cat(&args);
}

pub fn our_cat(args: &Vec<String>) -> (){

	use std::fs::File;
	use std::string::String;
	use std::io::{BufRead, BufReader};
	use std::io::ErrorKind;
	let mut len = args.len() ;
	if len == 1 {
<<<<<<< HEAD
		println!("{}","Error: No input".red());
=======
		println!("Error: No input".red());
>>>>>>> 011b21dfaee9f4b94cb771293165da05890c4cc5
		return
	}

	else {
		for x in 1..len {
			let filename = &args[x].trim();

			let file = match File::open(filename) {
				Ok(file) => file,
				Err(error) => match error.kind() {
					ErrorKind::NotFound => {
						println!("{} {}: {}", "Error: cat:".red().bold(), filename.red(), "No such file or directory".red());
						return
					}
					_other_error => {
						println!("{} {}: {}", "Error: cat:".red().bold(), filename.red(), "No such file or directory".red());
						return
					}
			  	},
			};

			// let file = match File::open(filename) {
			// 	Ok(file) => file,
			// 	Err(error) => match error.kind() {
			// 		ErrorKind::NotFound => match File::create("hello.txt") {
			// 		Ok(fc) => fc,
			// 		Err(e) => panic!("Problem creating the file: {:?}", e),
			// 		},
			// 		other_error => panic!("Problem opening the file: {:?}",
			// 		other_error),
			// 	},
			// };

			// let file = File::open(filename).unwrap_or_else( |error| {
			// 	if error.kind() == ErrorKind::NotFound {
			// 		println!("Error: cat: {}: file doesn't exist",filename);
			// 		return
			// 	}
			// 	else{
			// 		println!("Error: cat {}: Unexpected Error", filename);
			// 		return
			// 	}
			// });

			// let f = File::open("hello.txt").unwrap_or_else(|error| {
			// 	if error.kind() == ErrorKind::NotFound {
			// 	File::create("hello.txt").unwrap_or_else(|error| {
			// 	panic!("Problem creating the file: {:?}", error);
			// 	})
			// 	} else {
			// 	panic!("Problem opening the file: {:?}", error);
			// 	}
			// });


			let read = BufReader::new(file);

			for (index,line) in read.lines().enumerate(){
				let line = line.unwrap();{
					println!("{}",line);
				}
			}

		}

	}
}
