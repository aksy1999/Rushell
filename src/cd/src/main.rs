
fn main(){
	use std::env;
	let args: Vec<String> = env::args().collect();
	our_cd(&args);
}

// pub fn our_cd(args: &Vec<&str>) -> ()  {
pub fn our_cd(args: &Vec<String>) -> ()  {
	use std::{env};
	use std::path::Path;
	let current_dir = env::current_dir().unwrap();
	let mut path = String::from("");

	if args.len() == 1{
		return
	}

	if args[1].chars().next().unwrap() != '/'{
		path = current_dir.into_os_string().into_string().unwrap();
		path.push('/');
		path.push_str(&args[1]);
	}
	else {
		assert!(env::set_current_dir(&Path::new(&args[1])).is_ok());
		return
	}
	assert!(env::set_current_dir(&Path::new(&path)).is_ok());

	return
}
