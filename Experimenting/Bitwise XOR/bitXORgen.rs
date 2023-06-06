use std::io;

//Define the macro
macro_rules! BitwiseXOR {
	($x:expr, $y:expr) => {
			$x^$y
	}
}


//Calling the main exponent function
fn main(){

	// Read the first value from the user
    println!("Enter the first value:");
    let mut value1 = String::new();
    io::stdin().read_line(&mut value1).expect("Failed to read input"); 
    //The above line is important when we initialise a variable which we plan to use in the code
    //This line ensures that the variable that has been provided as an input is stored in the variable (Here it is stored in value1)
    
    let value1: i32 = value1.trim().parse().expect("Invalid input");
    /*This code line ensures that our input is 1. trim() - trimmed of any spaces 2. parse() - converted from a string to a 32 bit integer
    3. expect() - used to handle any errors that might occur during the parsing process. If parsing is successful, the parsed value is returned
    Else, the error message mentioned pops up */

    // Read the second value from the user
    println!("Enter the second value:");
    let mut value2 = String::new();
    io::stdin().read_line(&mut value2).expect("Failed to read input");
    let value2: i32 = value2.trim().parse().expect("Invalid input");


	let result = BitwiseXOR!(value1, value2); //Call the macro and save the output in 'result'
	println!("Bitwise XOR here is {}", result);
}