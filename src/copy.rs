pub fn our_copy(args: &Vec<&str>) -> (){
   use std::io::Read;
   use std::io::Write;   
   
   let source = args[1];
   let destination = args[2];
   let mut file_in = std::fs::File::open(source).unwrap();
   let mut file_out = std::fs::File::create(destination).unwrap();
   let mut buffer = [0u8; 4096];
   loop {
      let nbytes = file_in.read(&mut buffer).unwrap();
      file_out.write(&buffer[..nbytes]).unwrap();
      if nbytes <= 0 { 
      break; }
   }
}
