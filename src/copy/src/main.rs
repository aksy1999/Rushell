extern crate fs_extra;

// extern crate colour;
extern crate colored; // not needed in Rust 2018

use colored::*;
use fs_extra::dir::copy;
fn main(){
	use std::env;
	let args: Vec<String> = env::args().collect();
	our_copy(&args);
}

pub fn our_copy(args: &Vec<String>) -> (){
    use std::io::ErrorKind;
	use std::path::Path;
	use std::fs;
	// use colour::red;


    if args.len() == 1{
        println!("No source and destination files");
        return
    }

    if args.len() ==2 {
        println!("No destination file");
        return
    }

    let source = &args[1];
    let destination = &args[2];
	if Path::new(source).exists(){
		if Path::new(source).is_file(){
			// if Path::new(source).exists(){
			// 	if Path::new(source).is_dir(){
			// 		fs::copy(source, destination+"/"+filename);
			// 	}
			// }
			fs::copy(source, destination);
		}
		else if Path::new(source).is_dir(){
			let options = fs_extra::dir::CopyOptions::new();
			fs_extra::dir::copy(source, destination, &options);
		}
    }
	else{
		fs::copy(source, destination);
		// colour::blue("Invalid source");
		// println!("Invalid source");
		println!("{} {}: {}", "Error: cp:".red().bold(), source.to_string().red(), "No such file or directory".red());
	}
}
