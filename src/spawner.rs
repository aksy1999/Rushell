
use std::process::*;
use std::io::*;
use std::os::unix::io::AsRawFd;
use std::os::unix::io::FromRawFd;
use std::error::Error;
use std::clone::Clone;

pub fn pipe(args_l: Vec<Vec<String>>){

let n_c = args_l.len();
if n_c == 1{
	let _cmd1 = Command::new(&args_l[0][0]).args(&args_l[0][1..])
	.spawn()
	.expect("Failed to start ls process");
	let output1 = _cmd1.wait_with_output().expect("Failed to wait on cmd1");		
	println!("{}",String::from_utf8_lossy(output1.stdout.as_slice()));

}
if n_c == 2{
	
	if args_l[1].len()==0 {

		let output = Command::new(&args_l[0][0]).args(&args_l[0][1..])
		.output()
		.expect("Failed to execute command");
		println!("{}",String::from_utf8_lossy(output.stdout.as_slice()));
		//assert_eq!("{}\n", output.String::from_utf8_lossy(stdout.as_slice()));
		
	
	}
	else {
		let mut cmd = Command::new(&args_l[0][0]).args(&args_l[0][1..])
         	.stdout(Stdio::piped())
         	.spawn()
	 	.expect("Failed to start ls process");
		let cmd_out= cmd.stdout.expect("Failed to open ls stdout");
		//println!("{:?}",&args_l[1]);		
		
     		cmd = Command::new(&args_l[1][0]).args(&args_l[1][1..])
                 .stdin(Stdio::from(cmd_out))
		 .stdout(Stdio::piped())
		 .spawn()
		 .expect("error");
		let output = cmd.wait_with_output().expect("Failed to wait on cmd2");
    		println!("{}",String::from_utf8_lossy(output.stdout.as_slice()));
	}
}
if n_c ==3{
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
		println!("{}",String::from_utf8_lossy(output.stdout.as_slice()));

}

}

