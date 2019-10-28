// mod copy;
// mod remove;

fn main(){
	use std::env;
	let args: Vec<String> = env::args().collect();
	our_move(&args);
}

pub fn our_move(args: &Vec<String>) -> (){

	use std::io::Read;
    use std::io::Write;
	use std::io::ErrorKind;

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
    let mut file_in = match std::fs::File::open(source) {
		Ok(file_in) => file_in,
		Err(error) => match error.kind() {
			ErrorKind::NotFound => {
				println!("Error: mv: {}: No such file or directory", source);
				return
			}
			_other_error => {
				println!("Error: mv: {}: Unexpected Error", source);
				return
			}
		},
	};
	// let mut file_out = std::fs::File::create(destination);
    let mut file_out = match std::fs::File::create(destination) {
        Ok(file_in) => file_in,
		// Err(error) => {return}
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                println!("Error: mv: {}: Path to destination file does not exist", destination);
                return
            }
            _other_error => {
                println!("Error: mv: {}: Unexpected Error", destination);
                return
            }
        },
    };
    let mut buffer = [0u8; 4096];
    loop {
		// TODO: Move non-text files (files not in UTF-8 encoding)
       	let nbytes = file_in.read(&mut buffer).unwrap();
       	file_out.write(&buffer[..nbytes]).unwrap();
       	if nbytes <= 0 {
       	break; }
    }

	std::fs::remove_file(source).expect(" : Error: rm: File doesn't exist");
	println!{""};

}
