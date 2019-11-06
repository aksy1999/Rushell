
pub fn our_ls(args: &Vec<&str>) -> std::io::Result<()> {
	use std::fs;
	use std::path::Path;
	let mut path_v = "";
	if args.len() == 1 {
		path_v = "."
	}
	else {
		path_v = args[1];
	}

    for entry in fs::read_dir(path_v)? {
        let dir = entry?;
        println!("{:?}", dir.path());
    }
    Ok(())

}


