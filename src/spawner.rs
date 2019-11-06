
use std::process::*;
use std::io::*;
use std::os::unix::io::AsRawFd;
use std::os::unix::io::FromRawFd;
use std::error::Error;
use std::clone::Clone;
use std::process;
use std::panic;
extern crate colored; // not needed in Rust 2018

use colored::*;

pub fn pipe(args_l: Vec<Vec<String>>) -> std::io::Result<()> {

	let n_c = args_l.len();

	//This suppresses the panic verbose remove if required.

	panic::set_hook(Box::new(|_info| {
		// do nothing
	}));

	if n_c == 1{

		let result = panic::catch_unwind(|| {

			let mut _cmd1 = Command::new(&args_l[0][0]).args(&args_l[0][1..])
				.stdout(Stdio::inherit())
				.stderr(Stdio::inherit())
				.spawn()
				.expect("Failed to start ls process");

			// let output1 = _cmd1.wait_with_output().expect("Failed to wait on cmd1");
			// print!("{}",String::from_utf8_lossy(output1.stdout.as_slice()));
			let ecode = _cmd1.wait()
				.expect("failed to wait on child");
        	assert!(ecode.success());
        });

		if result.is_err(){
			// This will catch any error that occurs during the run of a command.
			println!("{}{}", &args_l[0][0].red().bold(), ": is not recognized as an internal or extrernal command, or operable program".red());
		}
	}



	if n_c == 2{

		if args_l[1].len()==0 {
			let result = panic::catch_unwind(|| {
			let output = Command::new(&args_l[0][0]).args(&args_l[0][1..])
				.output()
				.expect("Failed to execute command");
			print!("{}",String::from_utf8_lossy(output.stdout.as_slice()));
			});
			if result.is_err(){
				// This will catch any error that occurs during the run of a command.
				println!("{}{}", &args_l[0][0].red().bold(), ": is not recognized as an internal or extrernal command, or operable program".red());
			}

		}
		else {
			let result = panic::catch_unwind(|| {
			let mut cmd = Command::new(&args_l[0][0]).args(&args_l[0][1..])
         		.stdout(Stdio::piped())
     			.spawn()
		 		.expect("Failed to start ls process");
			let cmd_out= cmd.stdout.expect("Failed to open ls stdout");

     		cmd = Command::new(&args_l[1][0]).args(&args_l[1][1..])
            	.stdin(Stdio::from(cmd_out))
				.stdout(Stdio::piped())
				.spawn()
				.expect("error");
			let output = cmd.wait_with_output().expect("Failed to wait on cmd2");
	    		print!("{}",String::from_utf8_lossy(output.stdout.as_slice()));
			});
			if result.is_err(){
				// This will catch any error that occurs during the run of a command.
				println!("{}{}", &args_l[0][0].red().bold(), ": is not recognized as an internal or extrernal command, or operable program".red());
			}
		}
	}



	if n_c ==3{
		let result = panic::catch_unwind(|| {
			let mut cmd = Command::new(&args_l[0][0]).args(&args_l[0][1..])
				.stdout(Stdio::piped())
				.spawn()
				.expect("Failed to start ls process");
			let cmd_out= cmd.stdout.expect("Failed to open ls stdout");

			cmd = Command::new(&args_l[1][0]).args(&args_l[1][1..])
				.stdin(Stdio::from(cmd_out))
				.stdout(Stdio::piped())
				.spawn()
				.expect("error");
			let cmd_out_1= cmd.stdout.expect("Failed to open ls stdout");
			cmd = Command::new(&args_l[2][0]).args(&args_l[2][1..])
				.stdin(Stdio::from(cmd_out_1))
				.stdout(Stdio::piped())
				.spawn()
				.expect("error");

			let output = cmd.wait_with_output().expect("Failed to wait on cmd2");
			print!("{}",String::from_utf8_lossy(output.stdout.as_slice()));
		});

		if result.is_err(){
			// This will catch any error that occurs during the run of a command.
			println!("{}{}", &args_l[0][0].red().bold(), ": is not recognized as an internal or extrernal command, or operable program".red());
		}

	}
	Ok(())
}
