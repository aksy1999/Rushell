use std::fs::File;
use std::io::{BufRead, BufReader};

extern crate colored; // not needed in Rust 2018

use colored::*;
fn main(){
	use std::env;
	let args: Vec<String> = env::args().collect();
	our_cmp(&args);
}


pub fn our_cmp(args: &Vec<String>) {

	if args.len() < 2 {
		println!("Error: cmp: less input arguments");
	}
	else{
	    let filename = &args[1];
	    let file = File::open(filename).unwrap();
	    let reader = BufReader::new(file);

	    let filename1 = &args[2];
	    let file1 = File::open(filename1).unwrap();
	    let reader1 = BufReader::new(file1);
	    let mut flag = 0;
	    let mut ind=0;
	    let mut ind1=0;
			

	    for (index, line) in reader.lines().enumerate() {
		    let lined = line.unwrap(); 
		    let file1 = File::open(filename1).unwrap();
		    let reader1 = BufReader::new(file1);
		    for (index1, line1) in reader1.lines().enumerate() {
			        
			let line1d = line1.unwrap(); 
			ind=index;
			ind1=index1;		

			if (index == index1 && lined != line1d){
	        	println!("Differ");
			println!("Line {}",index+1);
						
			flag=1;
			break;
			}
		}
		if (flag==1)
		{
			break;
		}
	    }
	    if (ind != ind1){
	        	println!("No");
		}	
	}
}
