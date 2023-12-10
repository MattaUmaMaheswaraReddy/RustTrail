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

macro_rules! CaeserCipher {
    ($inputtext:expr, $privatekey:expr) => {{ 

        /* Why use '{{' in a macro instead of just '{'? The issue here is related to the macro generating a block with a let statement, 
        and in Rust, macro rules cannot directly generate variable declarations (like let statements) within expressions. The solution 
        is to use an expression-oriented syntax for your macro and that here is '{{' */

        let mut outputstring = String::new();

        for character in $inputtext.to_string().chars() {

            if character.is_alphabetic(){
                let character_to_lowercase = character.to_ascii_lowercase();

                let alphabet_number = character_to_lowercase as u8 - b'a' + 1;
                let new_alphabet_number = alphabet_number + $privatekey;

                let new_alphabet = ((new_alphabet_number - 1) % 26 + b'a') as char;
                outputstring.push(new_alphabet);
            }

            else {
                outputstring.push(character); //For pushing non-alphabetic characters such as spaces and others
            }
        }

        outputstring
    }}
}


fn main(){

	// Read the string from the user
    println!("Enter the string:");
    let mut inputstring = String::new();
    io::stdin().read_line(&mut inputstring).expect("Failed to read input"); 
    //The above line is important when we initialise a variable which we plan to use in the code
    //This line ensures that the variable that has been provided as an input is stored in the variable (Here it is stored in inputstring)
    

    // Read the key from the user
    println!("Enter the key value (between -26 to 26):");
    let mut key = String::new();
    io::stdin().read_line(&mut key).expect("Failed to read input");
    let key: u8 = key.trim().parse().expect("Invalid input");

    let result = CaeserCipher!(inputstring, key);


	println!("The Caeser Cipher for the above input is: {}", result);
}