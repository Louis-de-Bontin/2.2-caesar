use std::io;
use std::env;
use std::process;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Please provide the key, and only the key as command line argument.");
        process::exit(1);
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
