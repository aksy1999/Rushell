pub fn our_remove(args: &Vec<&str>) -> (){
   
   let source = args[1];
   std::fs::remove_file(source).expect("File doesn't exist");		  

}
