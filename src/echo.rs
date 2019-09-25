// Function for echo command
pub fn our_echo(args: &Vec<&str>) -> () {
	let mut pp = String::from("");
	if args.len() > 1{
		pp.push_str(args[1]);
	}
	for i in 2..args.len() {
		pp.push_str(" ");
		pp.push_str(args[i]);
	}
	println!("{} ", pp);
}