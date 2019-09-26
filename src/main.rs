mod copy;
mod remove;
mod cwd;
mod mkdir;
mod echo;
mod cd;
mod grep;
mod cat;
mod ls;
mod datetime;
// mod mov;

fn get_arg(char_vec: &Vec<char>, mut start: usize) -> (usize, usize, usize) {
	let mut comp = ' ';
	let mut arg_start = start;
	if char_vec[start] == '"'{
		comp = '"';
		start = start+1;
		arg_start = start;
	}
	while char_vec[start]!=comp {
		start = start+1;
	}
	let arg_stop = start;
	if comp=='"'{
		start = start+2;
	}
	else {
		start = start+1;
	}
	(start, arg_start, arg_stop)
}

fn get_args_len(char_vec: &Vec<char>) -> i32 {
	let mut len = 0;
	let mut i = 0;
	while i < char_vec.len() {
		if char_vec[i]=='"' {
			i = i+1;
			while char_vec[i]!='"' {
				i=i+1;
			}
		}
		if char_vec[i]==' ' {
			len = len + 1;
		}
		i=i+1;
	}
	len
}


fn start_process(args: &Vec<&str>) -> () {
	use std::process;
	use std::panic;

    //This suppresses the panic verbose remove if required.
	panic::set_hook(Box::new(|_info| {
        // do nothing
    }));

	let result = panic::catch_unwind(|| {
    	let mut child = process::Command::new(args[0])
    							.args(args)
    							.spawn()
    							.expect("command failed to start");
    	let ecode = child.wait()
    				.expect("failed to wait on child");
        assert!(ecode.success());
        });

	if result.is_err(){
		// This will catch any error that occurs during the run of a command.
		println!("'{}' is not recognized as an internal or extrernal command,\n or operable program", args[0]);
	}

}

fn our_exit(args: &Vec<&str>) -> () {
	use std::process;
	if args.len() == 1{
		process::exit(0);
	}
	else {
		let code:i32 = args[1].parse::<i32>().unwrap();
		process::exit(code);
	}
}

pub fn our_move(args: &Vec<&str>) -> (){
	use copy;
	use remove;
	copy::our_copy(&args);
	remove::our_remove(&args);
}

fn run_internal(args: &Vec<&str>) -> () {
	if args[0] == "exit" {
		our_exit(&args)
	}
	if args[0] == "cd" {
		cd::our_cd(&args)
	}
	if args[0] == "cwd" {
		cwd::our_cwd(&args);
	}
	if args[0] == "echo" {
		echo::our_echo(&args);
	}
	if args[0] == "mkdir" {
		mkdir::our_mkdir(&args);
	}
	if args[0] == "cp" {
		copy::our_copy(&args);
	}
	if args[0] == "rm" {
		remove::our_remove(&args);
	}
	if args[0] == "mv" {
		our_move(&args);
	}
	if args[0] == "grep" {
		grep::our_grep(&args);
	}
	if args[0] == "cat" {
		cat::our_cat(&args);
	}
	if args[0] == "ls" {
		ls::our_ls(&args);
	}
	if args[0] == "date" {
		datetime::date();
	}
}


fn set_prompt(args: &Vec<&str>) -> String {
	let mut pp = String::from("");
	if args.len() == 1 {
		pp.push_str("# ");
	}
	else {
		for i in 1..args.len()-1 {
			pp.push_str(args[i]);
			pp.push(' ');
		}
		pp.push_str(args[args.len()-1]);
	}
	pp
}


fn main() {
	use std::io::{stdin,stdout,Write};
	let mut prompt = String::from("# ");
	let mut internal_commands = Vec::new();
	
	internal_commands.push("exit");
	internal_commands.push("cd");
	internal_commands.push("cwd");
	internal_commands.push("echo");
	internal_commands.push("mkdir");
	internal_commands.push("cp");
	internal_commands.push("rm");
	internal_commands.push("mv");
	internal_commands.push("grep");
	internal_commands.push("cat");
	internal_commands.push("ls");
	internal_commands.push("date");
	loop {
		let mut command = String::from("");
		print!("{}",prompt);
		let _=stdout().flush();
		stdin().read_line(&mut command).expect("Invalid command");
		command = String::from(command.trim());

		command.push(' ');
		
		let char_vec: Vec<char> = command.chars().collect();
		let args_l = get_args_len(&char_vec).clone();
		let mut args:Vec<&str> = Vec::new();
		let mut start:usize = 0;

		for _i in 0..args_l {
			let (next, arg_start, arg_stop) = get_arg(&char_vec, start);
			args.push(&&command[arg_start..arg_stop]);
			start = next;
		}

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
