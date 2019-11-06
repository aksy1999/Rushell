// Funtion that makes a new directory

extern crate colored;
use colored::*;
fn main(){
	use std::env;
	let args: Vec<String> = env::args().collect();
	our_mkdir(&args);
}

pub fn our_mkdir(args: &Vec<String>) -> std::io::Result<()> {
	use std::fs;
	if args.len() < 2 {
		println!("{} {}", "Error: mkdir:".red().bold(), " no input arguments".red());
	}
	if args.len() == 2 {
		if args[1] == "-a"{
			println!("{} {}", "Error: mkdir:".red().bold(), "no input arguments".red());
		}
		// println!("Creating directory...");
	}
	else if args.len() > 2{
		// println!("Creating directories...");
		let mut start = 1;
		let mut all = 0;

		if args[1] == "-a"{
			start = 2;
			all = 1;
		}
		for i in start..args.len() {
			println!("  {}", &args[i]);
			if all==0{
		    	fs::create_dir(&args[i])?;
			}
			else{
				fs::create_dir_all(&args[i])?;
			}
	    }
	}
    Ok(())
}
