
//STILL NOT RUNNING - Check WHY

use std::io;

//Define the macro
macro_rules! BitwiseXOR {
	($x:expr, $y:expr) => {
			$x ^ $y
	}
}


//Calling the main exponent function
fn main(){

	// Read the first value from the user
    println!("Enter the first value:");
    let mut value1 = String::new();
    io::stdin().read_line(&mut value1).expect("Failed to read input");
    let value1: i32 = value1.trim().parse().expect("Invalid input");

    // Read the second value from the user
    println!("Enter the second value:");
    let mut value2 = String::new();
    io::stdin().read_line(&mut value2).expect("Failed to read input");
    let value2: i32 = value2.trim().parse().expect("Invalid input");


	//Invoke the macro 
    BitwiseXOR!(value1, value2);

	let result = BitwiseXOR();
	println!("Bitwise XOR here is {}", result);
}