pub fn our_cat(args: &Vec<&str>) -> (){

	use std::fs::File;
	use std::string::String;
	use std::io::{BufRead, BufReader};
	use std::io::ErrorKind;
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
				Err(error) => match error.kind() {
					ErrorKind::NotFound => {
						println!("Error: cat: {}: No such file or directory",filename);
						return
					}
					other_error => {
						println!("Error: cat {}: Unexpected Error", filename);
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
