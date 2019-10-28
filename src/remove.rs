fn main(){
	use std::env;
	let args: Vec<String> = env::args().collect();
	our_remove(&args);
}

pub fn our_remove(args: &Vec<String>) -> (){
    use std::io::ErrorKind;

	if args.len() == 1 {
		println!("Error: rm: no input arguments");
	}
	if args.len() == 2 {
		println!("Removing file ...");
	}
	else {
		println!("Removing files ...");
	}
	for i in 1..args.len(){
		let source = &args[i];
		print!("  {}", &args[i]);
		// std::fs::remove_file(source).expect(" : Error: rm: File doesn't exist");
        match std::fs::remove_file(source) {
            Ok(file_in) => file_in,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => {
                    println!("Error: rm: {}: Path to destination file does not exist", source);
                    return
                }
                _other_error => {
                    println!("Error: rm: {}: Unexpected Error", source);
                    return
                }
            },
        };
		println!{""};
	}
}
