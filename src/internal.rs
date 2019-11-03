use std::collections::HashMap;
use std::env;
use std::path::Path;

pub fn init_stack() -> HashMap<String, String> {
	let mut stack = HashMap::new();
	for (key, value) in env::vars() {
		stack.insert(key, value);
	}
	let path = env::current_dir().unwrap();
	stack.insert(String::from("CWD"), String::from(format!("{}", path.display())));
	stack.insert(String::from("PROMPT"), String::from("# "));
	stack
}

pub fn set(key: String, value:String, stack: &mut HashMap<String, String>) -> () {
	stack.insert(key.clone(), value.clone());
	for (rkey, _value) in env::vars() {
		if rkey==key {
			env::set_var(&key, &value);
		}
	}
}

pub fn unset(rkey: String, stack: &mut HashMap<String, String>) -> () {
	stack.remove(&rkey);
}

pub fn our_cd(args: Vec<String>) -> String  {
	let current_dir = env::current_dir().unwrap();
	let mut path = String::from("");

	if args.len() == 1{
		path = current_dir.into_os_string().into_string().unwrap();
		return path;
	}

	if args[1].chars().next().unwrap() != '/'{
		path = current_dir.into_os_string().into_string().unwrap();
		path.push('/');
		path.push_str(&args[1]);
	}

	assert!(env::set_current_dir(&Path::new(&path)).is_ok());

	let current_dir = env::current_dir().unwrap();
	path = current_dir.into_os_string().into_string().unwrap();
	return path;
}

pub fn our_exit(args: Vec<String>) -> () {
	use std::process;
	if args.len() == 1{
		process::exit(0);
	}
	else {
		let code:i32 = args[1].parse::<i32>().unwrap();
		process::exit(code);
	}
}

pub fn set_prompt(args: Vec<String>) -> String {
	let mut pp = String::from("");
	if args.len() == 1 {
		pp.push_str("# ");
	}
	else {
		for i in 1..args.len()-1 {
			pp.push_str(&args[i]);
			pp.push(' ');
		}
		pp.push_str(&args[args.len()-1]);
	}
	pp
}
