use std::collections::HashMap;
use std::io::{stdin,stdout,Write};

mod spawner;
mod internal;
mod parser;


// This function takes care of running internal commands
fn run_internal(
	args: Vec<Vec<String>>,
	stack: &mut HashMap<String, String>,
	internal_commands: &Vec<String>
	) -> String {

	// For noting down the status of internal command
	let mut status = "ok";

	// Calls exit command
	if &args[0][0] == "exit" {
		internal::our_exit(args[0].clone());
	}
	// Calls cd
	if &args[0][0] == "cd" {
		internal::set(String::from("CWD"), internal::our_cd(args[0].clone()).clone(), stack);
	}
	// Calls clear (works only in Ubuntu)
	if &args[0][0] == "clear" {
		println!("\x1B[2J");
	}
	// Sets a variable
	if &args[0][0] == "set" {
		if args[0].len() != 3{
			status = "Error";
			println!("Format is set <variable> <value>");
		}
		else {
			internal::set(args[0][1].clone(), args[0][2].clone(), stack);
		}
	}
	// For setting the prompt
	if &args[0][0] == "prompt" {
		internal::set(String::from("PROMPT"), internal::set_prompt(args[0].clone()).clone(), stack);
	}
	//Unset a variable
	if &args[0][0] == "unset" {
		if args[0].len() != 2 {
			status = "Error";
			println!("Format is unset <varibale>");
		}
		else {
			internal::unset(args[0][1].clone(), stack);
		}
	}
	// Runs a for loop
	if &args[0][0] == "for" {
		if args[0].len() != 4{
			status = "Error";
			println!("Format for <var> <lower> <upper>");
		}
		else {
			//Setting lower and upper limit [a, b)
			let lower: i32 = args[0][2].parse().unwrap();
			let upper: i32  = args[0][3].parse().unwrap();
			if lower+1 >= upper {
				status = "Error";
				// return status from here
				return String::from(status);
			}
			// Getting the command to loop
			print!(": ");
			let mut command = String::from("");
			let _=stdout().flush();
			stdin().read_line(&mut command).expect("Error taking Input");

			for i in lower..upper{
				internal::set(args[0][1].clone(), String::from(i.to_string()), stack);
				let execute = executer(command.clone(), &internal_commands, stack);
				// Break if there is error
				if execute == "Error"{
					break;
				}
			}
			// Unset the variable after for loop is completed
			internal::unset(args[0][1].clone(), stack);

		}
	}
	// Return the status
	String::from(status)
}

// This function takes care of parsing, distinguishing the command
// and then execute it
// It returns it execute state as string

fn executer(
	command: String, 
	internal_commands: &Vec<String>, 
	stack: &mut HashMap<String, String>
	) -> String {
		let mut command = command;
		for (key, value) in stack.clone() {
			command = command.replace(&format!("${}", key),&value);
		}

		let command = String::from(command);

		let args_list = parser::parser(command);
		let mut execute = "external";

		if args_list.len() == 0{
			return String::from(execute);
		}

		if &args_list[0][0]==""{
			return String::from(execute);
		}

		let args_len = args_list.len();

		for i in 0..args_len{
			for com in internal_commands {
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
			if &run_internal(args_list.clone(), stack, &internal_commands) == "Error"{
				execute = "Error";
			}
		}
		if execute=="external" {
			spawner::pipe(args_list);
		}
		String::from(execute)
}


// This is the main loop that will take commands
fn main() {
	//Initiate the stack
	let mut stack = internal::init_stack();
	//Initiate the internal commands
	let mut internal_commands = Vec::new();
	
	internal_commands.push(String::from("exit"));
	internal_commands.push(String::from("cd"));
	internal_commands.push(String::from("clear"));
	internal_commands.push(String::from("set"));
	internal_commands.push(String::from("unset"));
	internal_commands.push(String::from("prompt"));
	internal_commands.push(String::from("for"));

	// Loop for taking commands
	loop {
		let mut command = String::from("");
		print!("{}",stack["PROMPT"]);

		let _=stdout().flush();
		stdin().read_line(&mut command).expect("Error taking Input");

		executer(command, &internal_commands, &mut stack);
	}
}
