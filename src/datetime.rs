pub fn date() -> (){
	extern crate chrono;
	// use chrono;
	println!("{:?}", chrono::offset::Local::now());
}

// pub fn sleep(args: &Vec<&str>) -> (){
// 	use std::time::{Duration, SystemTime};
// 	use std::thread::sleep;
// 	// let sleep_duration = args[1].parse::<u64>();
// 	let sleep_duration = i32::from_str(s).unwrap();
// 	sleep(Duration::new(sleep_duration, 0));
// }


// use std::time::SystemTime;

// let d = SystemTime::now();
// println!("{:?}",d);

// extern crate chrono;
// use chrono::{DateTime, Duration, Utc};
// use chrono;

// let now = Utc::now();
//    println!("{}", now);

//    println!("{:?}", chrono::offset::Local::now());