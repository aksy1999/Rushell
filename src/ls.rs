extern crate chrono;
extern crate structopt;

use std::fs;
use std::path::PathBuf;
use std::error::Error;
use std::process;
use structopt::StructOpt;
use chrono::{DateTime, Local};

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(default_value = ".", parse(from_os_str))]
    path: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    if let Err(ref e) = run(&opt.path) {
        println!("{}", e);
        process::exit(1);
    }
}

fn run(dir: &PathBuf) -> Result<(), Box<Error>> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let data = entry.metadata()?;
            let file_name = entry.file_name().into_string().or_else(|f| Err(format!("Invalid entry: {:?}", f)))?;
            let file_size = data.len();
            let LAST: DateTime<Local> = DateTime::from(data.modified()?);
            println!("{:>5} {} {}",file_size,LAST.format("%_d %b %H:%M").to_string(),file_name);
        }
    }
    Ok(())
}
