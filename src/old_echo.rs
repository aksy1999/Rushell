// Function for echo command
fn main(){
	use std::env;
	let args: Vec<String> = env::args().collect();
	our_echo(&args);
}

pub fn our_echo(args: &Vec<String>) -> () {
	let mut pp = String::from("");
	if args.len() > 1{
		pp.push_str(&args[1]);
	}
	for i in 2..args.len() {
		pp.push_str(" ");
		pp.push_str(&args[i]);
	}
	println!("{} ", pp);
}
