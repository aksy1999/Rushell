pub fn our_remove_single(args: &Vec<&str>) -> (){
   
	if args.len() == 1 {
		println!("Error: rm: no input arguments");
	}
	else {
		let source = args[1];
		std::fs::remove_file(source).expect(" : Error: rm: File doesn't exist");	
		println!{""};
	}
}
