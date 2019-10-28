// Function to get the current working directory
fn main(){
	use std::env;
	let args: Vec<String> = env::args().collect();
	our_cwd(&args);
}

pub fn our_cwd(args: &Vec<String>)-> std::io::Result<()> {
	use std::env;
    let path = env::current_dir()?;
    println!("{}", path.display());
    //process::exit(code);
    Ok(())
}
