//Define the macro
/*
macro_rules! XOR {
	($x:expr) => {
		fn XOR() -> i32{
			$x ^ 3
		}
	}
}
*/

macro_rules! XOR {
	($x:expr) => {
		
			$x ^ 3
		
	}
}

//Invoke the macro 
//XOR!(4);

//Calling the main exponent function
fn main(){
	let result = XOR!(4);
	println!("Bitwise XOR here is {}", result);
}