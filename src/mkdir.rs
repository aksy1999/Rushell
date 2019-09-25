// Funtion that makes a new directory
pub fn our_mkdir(args: &Vec<&str>) -> std::io::Result<()> {
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
		println!("  {}", args[i]);
    	fs::create_dir(args[i])?;
    }
    Ok(())
}