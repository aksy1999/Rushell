fn start_process(command: &str) -> () {
	use std::process;
	let split = command.split(" ");
	let args = split.collect::<Vec<&str>>();
	let mut child = process::Command::new(args[0])
							.args(args)
							.spawn()
							.expect("command failed to start");
	let ecode = child.wait()
				.expect("failed to wait on child");
    assert!(ecode.success());
}

fn main() {
	use std::io::{stdin,stdout,Write};
	use std::process;
	loop {
		let mut command = String::new();
		print!("# ");
		let _=stdout().flush();
		stdin().read_line(&mut command).expect("Invalid command");
		let command = command.trim();
		if command == "exit" {process::exit(0);}
		if command != ""
		{
			start_process(command);
		}
	}

}
