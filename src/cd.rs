pub fn our_cd(args: &Vec<&str>) -> ()  {
	use std::{env};
	use std::string::String;
	use std::path::Path;
	let current_dir = env::current_dir().unwrap();
	let mut path = String::from("");

	if args.len() == 1{
		return
	}

	if args[1].chars().next().unwrap() != '/'{
		path = current_dir.into_os_string().into_string().unwrap();
	}
	else {
		assert!(env::set_current_dir(&Path::new(args[1])).is_ok());
		return
	}
	
	let mut vector_path: Vec<&str> = path.split("/").collect();
	let _pop = vector_path.pop();
	
	if vector_path.len() == 1{
		let error_path = format!("{}{}",path,"/");
		let root = Path::new(&error_path);
		assert!(env::set_current_dir(&root).is_ok());
		return		
	}
	
	let mut a = vec![];
	let mut b;
	for mum in &vector_path {
		b = format!("{}{}",mum,"/");
		a.push(b);
	}
	let final_string: String = a.into_iter().collect();

	if args[1].trim() == ".." || args[1].trim() == "../"{
		let root = Path::new(&final_string);
		assert!(env::set_current_dir(&root).is_ok());
		
	}
	
	return
}



