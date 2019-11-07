// use sysinfo;
extern crate sysinfo;
fn main(){

	use std::env;
	let args: Vec<String> = env::args().collect();
    use sysinfo::{ProcessExt, SystemExt};

    let mut system = sysinfo::System::new();

    // First we update all information of our system struct.
    system.refresh_all();

    // Now let's print every process' id and name:
    if args.len() == 2{
        if args[1] == "-f"{
            let mut i = 0;
            for (pid, proc_) in system.get_process_list() {
                println!("{}:{} => status: {:?}", pid, proc_.name(), proc_.status());
                i += 1;
                if (i == 10){
                    break;
                }
            }

        }
        else if args[1] == "-m"{
            // And finally the RAM and SWAP information:
            println!("total memory: {} kB", system.get_total_memory());
            println!("used memory : {} kB", system.get_used_memory());
            println!("total swap  : {} kB", system.get_total_swap());
            println!("used swap   : {} kB", system.get_used_swap());
        }
        else{
            for (pid, proc_) in system.get_process_list() {
                println!("{}:{} => status: {:?}", pid, proc_.name(), proc_.status());
            }
        }
    }
    else{
        for (pid, proc_) in system.get_process_list() {
            println!("{}:{} => status: {:?}", pid, proc_.name(), proc_.status());
        }
    }


    // // Then let's print the temperature of the different components:
    // for component in system.get_components_list() {
    //     println!("{:?}", component);
    // }
    //
    // // And then all disks' information:
    // for disk in system.get_disks() {
    //     println!("{:?}", disk);
    // }

}
// https://crates.parity.io/sysinfo/index.html
