use std::io;                                                                               // I'm using the function to ask user input
use std::env;                                                                              // I'm using the function to get command line arguments
use std::process;                                                                          // I'm using the exit function that exit the program with an exit status


fn main() {
    let args: Vec<String> = env::args().collect();                                         // Get command line arguments and put them in a vector of String
    if args.len() != 2 {
        println!("Please provide the key, and only the key as command line argument.");
        process::exit(1);                                                                  // Exit program with status 1
    }

    let key = string_to_int(&args[1]);
    let plain_text = input("Plain text : ");

    println!("{}", encrypt(&plain_text, &key));
}

fn input(message: &str) -> String {
    // This function displays an informative string to the user, and return his input
    println!("{}", message);
    let mut user_input = String::from("");
    io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read input.");
    user_input
}

fn string_to_int(nb_str: &str) -> u8 {
    // Transform a string into an unsigned int, or exit if failed.
    let nb: u8 = match nb_str.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please provide a correct number.");
            process::exit(1);
        },
    };
    nb
}

fn encrypt(plain_text: &String, key: &u8) -> String {
    // Encrypt the plain text by moving each char by *key* ranks. Preserve the case and the non lettre chars.
    let mut cypher = String::from("");
    for mut char in plain_text.chars() {
        let unicode: u8 = char as u8;
        if unicode <= 122 && unicode >= 97 {                                          // if is a lowercase lettre
            char = (97 + ((unicode - 97 + key)%26)) as char;
        }
        if unicode <= 90 && unicode >= 65 {                                           // if is a uppercase lettre
            char = (65 + ((unicode - 65 + key)%26)) as char;
        }
        cypher.push(char);
    }
    cypher
}
