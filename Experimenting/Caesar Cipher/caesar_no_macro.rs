/* How does the Caesar cipher work? You basically take an alphabet and select a number by
which you wanna shift the alphabet by - either to the right or to the left. 

How do you get started? Firstly, the user has to input a string of characters and a number. 
Once we have those details, we have to give an output which would shift this string n postions
either to the right (if n is negative, then we shift it to the left) */

use std::io;

//Define the macro

/* When we choose x, which is a string, I want it to break it down to single characters,
convert those characters to numbers (a = 1, b = 2, c = 3, etc.). Then do addition operation,
get the new numbers and then again convert these numbers to characters and then concatenate
them to get the output string. */


fn main(){

	// Read the string from the user
    println!("Enter the string:");
    let mut inputstring = String::new();
    io::stdin().read_line(&mut inputstring).expect("Failed to read input"); 
    //The above line is important when we initialise a variable which we plan to use in the code
    //This line ensures that the variable that has been provided as an input is stored in the variable (Here it is stored in inputstring)
    

    let mut outputstring = String::new();

    // Read the key from the user
    println!("Enter the key value (between -26 to 26):");
    let mut key = String::new();
    io::stdin().read_line(&mut key).expect("Failed to read input");
    let key: u8 = key.trim().parse().expect("Invalid input");

    for c in inputstring.to_string().chars() { //Here inputstring.chars() won't work because the inputstring is of type i32 (signed integer).
    // We need to convert it to a string first before using chars().
    	if c.is_alphabetic(){
    		let c_to_lowercase = c.to_ascii_lowercase();

    		let alphabet_number = c_to_lowercase as u8 - b'a' + 1;
    		let new_alphabet_number = alphabet_number + key; //Rust has strict type-checking rules. Does not allow us to operate on two different data types
    		// That is why we convert alphabet_number to i32 (or change key to u8 when we declare it) before adding it to the key. 

    		let new_alphabet = ((new_alphabet_number - 1) % 26 as u8 + b'a') as char;

    		outputstring.push(new_alphabet);

    	}
    }

	println!("The Caeser Cipher for the above input is: {}", outputstring);
}