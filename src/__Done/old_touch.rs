// Funtion that makes a new directory
fn main(){
	use std::env;
	let args: Vec<String> = env::args().collect();
	our_touch(&args);
}

pub fn our_touch(args: &Vec<String>) -> std::io::Result<()> {
	use std::fs::File;
    // use std::io::ErrorKind;
	if args.len() < 2 {
		println!("Error: touch: no input arguments");
	}
	else {
		for i in 1..args.len() {
			let _file_in = match std::fs::File::open(&args[i]) {
		        Ok(_file_in) => _file_in,
				Err(_error) => {
					let file = File::create(&args[i])?;
					file
				},
		        // Err(error) => match error.kind() {
		        //     ErrorKind::NotFound => {
		        //         file = File::create(&args[i])?;
		        //         file
		        //     }
		        //     _other_error => {
		        //         println!("Error: touch: Unexpected Error");
		        //         error
		        //     }
		        // },
		    };
	    }
	}
    Ok(())
}
