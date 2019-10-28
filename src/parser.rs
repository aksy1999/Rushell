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
	let char_len = char_vec.len();
	while i < char_len {
		if char_vec[i]=='"' {
			i = i+1;
			while char_vec[i]!='"' {
				if i==char_len-1{
					println!("Unbalanced Quotes");
					return 0
				}
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


pub fn parser(command: String) -> Vec<Vec<String>> {
	let mut pipest: Vec<String> = Vec::new();

	for com in command.split("|"){
		pipest.push(String::from(format!("{}{}", com.trim(), " ")));
	}

	// Convert Vec<String> to Vec<&str>
	let pipes: Vec<&str> = pipest.iter().map(AsRef::as_ref).collect();

	let mut args_arr: Vec<Vec<String>> = Vec::new();
	for com in pipes{
		let char_vec: Vec<char> = com.chars().collect();
		let args_l = get_args_len(&char_vec).clone();
		let mut args:Vec<String> = Vec::new();
		let mut start:usize = 0;

		for _i in 0..args_l {
			let (next, arg_start, arg_stop) = get_arg(&char_vec, start);
			args.push(String::from(&com[arg_start..arg_stop]));
			start = next;
		}
		args_arr.push(args)
	}
	args_arr
}
