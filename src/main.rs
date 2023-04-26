use std::io;
use rand::Rng;

fn main() {
    let pass_len = loop { // stores the result of the loop in "pass_len"
        println!("Please enter the length of your password (8-32): ");

        let pass_len = read_int(); // calls read int method

        if pass_len >= 8 && pass_len <= 32 { // checks for valid length
            break pass_len // breaks out of loop and returns the pass_len variable
        } else {
            println!("{} is not valid.", pass_len);
        }
    };

    let char_str = create_char_str(); // stores the string from which the characters get picked randomly

    let pass = gen_pass(pass_len, char_str); // stores the generated password

    println!("\nYour password: {}", pass);
}

// generates the password
fn gen_pass(pass_len: u8, char_str:String) -> String{
    let mut pass = String::with_capacity(pass_len as usize); // creates a new String with the fixed size of "pass_len"

    while pass.len() < pass_len as usize { // while the the already generated password is shorter than the desired length
        let rand_index = rand::thread_rng().gen_range(0..char_str.len()); // generates a random character index in the "char_str" string

        pass.push(char_str.chars().nth(rand_index).unwrap()); // adds the character of the "char_str" string at the "rand_index" index to the "pass" string
    }

    pass // returns the finished password
}

fn create_char_str() -> String { // creates the tha character string from which the characters will be picked randomly
    let mut char_str = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz");

    loop { // loops until input is valid
        println!("\nShould it include numbers? [y/n]: ");

        let input = read_str().to_lowercase(); // reads input and puts it into lowercase

        if input == "y" {
            char_str.push_str("0123456789"); // adds numbers to the "char_str" string
            break;
        } else if input == "n" {
            break;
        } else {
            println!("{input} is not a valid input.");
        }
    }

    // same as before
    loop {
        println!("\nShould it include special characters? [y/n]: ");

        let input = read_str().to_lowercase();

        if input == "y" {
            char_str.push_str(".,;:\"'()$%&{}[]+*#-_<>/\\");
            break;
        } else if input == "n" {
            break;
        } else {
            println!("{input} is not a valid input.");
        }
    }

    char_str // return the finished character string
}

// reads a string input
fn read_str() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");

    let input = input.trim().to_string(); // removes newline character and converts to string
    input // returns input
}

// reads a int input
fn read_int() -> u8 {
    loop { // loops until input is valid
        let mut input = String::new(); // creates new string input

        // reads input and stores it in the "input" variable
        io::stdin()
            .read_line(&mut input)
            .expect("Failed reading input");

        // trims the input (=removes newline characters)
        let input = input.trim();

        // tries to parse the "input" string to an u8
        match input.parse::<u8>() {
            Ok(input) => return input, // if there were no errors, return input
            Err(..) => println!("{} is not a valid number.", input) // if there were any errors, inform user
        };
    }
}
