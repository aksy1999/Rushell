use std::fs::File;
use std::io::{BufRead, BufReader};


fn main() {
    let filename = "src/ad.rs";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let filename1 = "src/da.rs";
    let file1 = File::open(filename1).unwrap();
    let reader1 = BufReader::new(file1);
    let mut flag = 0;
    let mut ind=0;
    let mut ind1=0;
		

    for (index, line) in reader.lines().enumerate() {
	    let lined = line.unwrap(); 
	    let file1 = File::open(filename1).unwrap();
	    let reader1 = BufReader::new(file1);
	    for (index1, line1) in reader1.lines().enumerate() {
		        
		let line1d = line1.unwrap(); 
		ind=index;
		ind1=index1;		

		if (index == index1 && lined != line1d){
        	println!("Differ");
		println!("Line {}",index+1);
					
		flag=1;
		break;
		}
	}
	if (flag==1)
	{
		break;
	}
    }
    if (ind != ind1){
        	println!("No");
	}	
}
