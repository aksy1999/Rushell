mod ls;
fn main(){
    use std::path::Path;
    if Path::new("main.rs").exists(){
        println!("Path exists");
        if Path::new("./main.rs").is_dir(){
            println!("Directory");
        }
        else{
            println!("Not a directory");
        }
        if Path::new("./main.rs").is_file(){
            println!("File");
        }
        else{
            println!("Not a file");
        }
    }
    else{
        println!("Path does not exist");
    }
    
    println!("{:?}",ls::our_ls(&vec!["ls","."]));
}
