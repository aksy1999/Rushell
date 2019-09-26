pub fn our_remove_single(args: &Vec<&str>) -> (){
   
	if args.len() == 1 {
		println!("Error: rm: no input arguments");
	}
	if args.len() == 2 {
		println!("Removing file ...");
	}
	else {
		println!("Removing files ...");
	}
	for i in 1..args.len(){
		let source = args[i];
		print!("  {}", args[i]);
		std::fs::remove_file(source).expect(" : Error: rm: File doesn't exist");	
		println!{""};
	}
}
