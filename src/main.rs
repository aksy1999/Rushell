fn start_process(args: &Vec<&str>) -> () {
	use std::process;
	let mut child = process::Command::new(args[0])
							.args(args)
							.spawn()
							.expect("command failed to start");
	let ecode = child.wait()
				.expect("failed to wait on child");
    assert!(ecode.success());
}

fn our_exit(args: &Vec<&str>) -> () {
	use std::process;
	if args.len() == 1{
		process::exit(0);
	}
	else {
		let code = args[1].parse().unwrap();
		process::exit(code);
	}
}

fn run_internal(args: &Vec<&str>) -> () {
	if args[0] == "exit" {
		our_exit(&args)
	}
}

fn set_prompt(args: &Vec<&str>) -> String {
	let mut pp = String::from("");
	if args.len() == 1 {
		pp.push_str("# ");
	}
	else {
		for i in 1..args.len() {
			pp.push_str(args[i]);
			pp.push_str(" ");
		}
	}
	pp
}


fn main() {
	use std::io::{stdin,stdout,Write};
	let mut prompt = String::from("# ");
	let mut internal_commands = Vec::new();
	
	internal_commands.push("exit");
	loop {
		let mut command = String::from("");
		print!("{}",prompt);
		let _=stdout().flush();
		stdin().read_line(&mut command).expect("Invalid command");
		command = String::from(command.trim());
		let split = command.split(" ");
		let args = split.collect::<Vec<&str>>();

		let mut internal = false;

		if args[0] == "" {continue;}
		if args[0] == "prompt"{
			prompt = set_prompt(&args).clone();
			continue;
		}
		for com in &internal_commands{
			if args[0] == *com {
				internal = true;
				run_internal(&args);
				break;
			}
		}
		if internal == false {
			start_process(&args)
		}
	}

}
