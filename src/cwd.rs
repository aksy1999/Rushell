// Function to get the current working directory
pub fn our_cwd(args: &Vec<&str>)-> std::io::Result<()> {
	use std::env;
    let path = env::current_dir()?;
    println!("{}", path.display());
    //process::exit(code);
    Ok(())
}