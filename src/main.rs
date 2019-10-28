use std::collections::HashMap;

mod internal;
mod parser;

fn run_internal(args: Vec<Vec<String>>, stack: &mut HashMap<String, String>) -> () {
	if &args[0][0] == "exit" {
		internal::our_exit(args[0].clone());
	}
	if &args[0][0] == "cd" {
		internal::set(String::from("CWD"), internal::our_cd(args[0].clone()).clone(), stack);
	}
	if &args[0][0] == "clear" {
		print!("\x1B[2J");
	}
	if &args[0][0] == "set" {
		if args[0].len() != 3{
			println!("Format is set <variable> <value>");
		}
		else {
			internal::set(args[0][1].clone(), args[0][2].clone(), stack);
		}
	}
	if &args[0][0] == "prompt" {
		internal::set(String::from("PROMPT"), internal::set_prompt(args[0].clone()).clone(), stack);
	}
	if &args[0][0] == "unset" {
		if args[0].len() != 2 {
			println!("Format is unset <varibale>");;
		}
		else {
			internal::unset(args[0][1].clone(), stack);;
		}
	}
}

fn main() {
	use std::io::{stdin,stdout,Write};
	let mut stack = internal::init_stack();
	let mut internal_commands = Vec::new();
	
	internal_commands.push(String::from("exit"));
	internal_commands.push(String::from("cd"));
	internal_commands.push(String::from("clear"));
	internal_commands.push(String::from("set"));
	internal_commands.push(String::from("unset"));
	internal_commands.push(String::from("prompt"));

	loop {
		let mut command = String::from("");
		print!("{}",stack["PROMPT"]);

		let _=stdout().flush();
		stdin().read_line(&mut command).expect("Error taking Input");

		for (key, value) in &stack {
			command = command.replace(&format!("${}", key),&value);
		}

		command = String::from(command);

		let args_list = parser::parser(command);

		if &args_list[0][0]==""{
			continue;
		}
		let mut execute = "external";

		let args_len = args_list.len();

		for i in 0..args_len{
			for com in &internal_commands {
				if &&args_list[i][0] == &com{
					if args_len > 1 {
						println!("Internal Commands cannot be piped");
						execute = "Error";
						break;
					}
					else {
						execute = "internal";
						break;
					}
				}
			}
			if execute!="external"{
				break;
			}
		}

		if execute=="internal" {	
			run_internal(args_list.clone(), &mut stack);
		}
		if execute=="external" {
			println!("{:?}", args_list.clone());
		}
		
	}
}
