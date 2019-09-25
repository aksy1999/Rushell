// Function for changing directory
pub fn our_cd(args: &Vec<&str>) -> () {
	//use std::process;
	//use std::env;
	let mut pp = String::from("");
	
	if args.len() == 1{
		pp.push_str("# ");
	}
	if args.len() == 2{
		pp.push_str("# ");
		//let code = args[1].parse().unwrap();
		pp.push_str(args[1]);
		//match env::set_current_dir(&code) 
		}
	
}
