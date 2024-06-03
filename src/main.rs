#[allow(unused_imports)]
use std::io::{self, Write};

fn handle_command(input: &str) -> bool {

    let parts: Vec<&str> = input.split_whitespace().collect();

    match parts[0] {
        "exit" => {
            if parts.len() == 1 || (parts.len() > 1 && parts[1] == "0") {
                return true;
            } else {
                println!("{}: command not found", input);
            }
        },
        "echo" => println!("{}", &input[5..]),
        "type" => {
            if parts.len() > 1 && (parts[1] == "exit" || parts[1] == "echo" || parts[1] == "type") {
                println!("{} is a shell builtin", parts[1]);
            } else {
                println!("{}: not found", parts[1]);
            }
        },
        _ => println!("{}: command not found", input),
    }

    return false
}

fn main() {
    print!("$ ");
    io::stdout().flush().unwrap();

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if handle_command(input) {
            break;
        }
        print!("$ ");
        io::stdout().flush().unwrap();
    }
}
