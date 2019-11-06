// Funtion that makes a new directory
fn main(){
	use std::env;
	let args: Vec<String> = env::args().collect();
	our_mkdir(&args);
}

pub fn our_mkdir(args: &Vec<String>) -> std::io::Result<()> {
	use std::fs;
	if args.len() < 2 {
		println!("Error: mkdir: no input arguments");
	}
	if args.len() == 2 {
		println!("Creating directory...");
	}
	else {
		println!("Creating directories...");
	}
	for i in 1..args.len() {
		println!("  {}", &args[i]);
		if 1==1{
	    	fs::create_dir(&args[i])?;
		}
		else{
			fs::create_dir_all(&args[i])?;
		}
    }
    Ok(())
}
