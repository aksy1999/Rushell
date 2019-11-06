extern crate fs_extra;
use fs_extra::dir::copy;
fn main(){
	use std::env;
	let args: Vec<String> = env::args().collect();
	our_copy(&args);
}

pub fn our_copy(args: &Vec<String>) -> (){
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
	if Path::new(source.clone()).exists(){
		if Path::new(source.clone()).is_file(){
			fs::copy(source, destination);
		}
		else if Path::new(source.clone()).isdir(){
			let options = CopyOptions::new()
			fs_extra::dir::copy(source, destination, &options);
		}
    }
	else{
		println!("Invalid source");
	}
}
