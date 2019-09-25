// mod copy;
// mod remove;
pub fn our_move(args: &Vec<&str>) -> (){
	use copy;
	use remove;
	copy::our_copy(&args);
	remove::our_remove(&args);
}
