pub fn our_cat(args: &Vec<&str>) -> (){
	
	use std::fs::File;
	use std::string::String;
	use std::io::{BufRead, BufReader};
	
	let mut len = args.len() ;
	if len == 1 {
		println!("Error: No input");
		return
	}
	
	else {
		for x in 1..len {
			let filename = args[x].trim();
			let file = match File::open(filename) {
				Ok(file) => file,
				Err(e) => { 
						println!("{}: file doesn't exist",filename);
						return
					  }
			};	
			let read = BufReader::new(file);

			for (index,line) in read.lines().enumerate(){
				let line = line.unwrap();{			
					println!("{}",line);
				}
			}

		}
		
	}
}


