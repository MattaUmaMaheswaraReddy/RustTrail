//Define the macro


macro_rules! XOR {
	($x:expr) => {
			$x ^ 3
	}
}


//Calling the main exponent function
fn main(){
	let result = XOR!(4);
	println!("Bitwise XOR here is {}", result);
}