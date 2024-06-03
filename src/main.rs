#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        let stdin = io::stdin();
        let mut input = String::new();
        print!("$ ");
        io::stdout().flush().unwrap();
        stdin.read_line(&mut input).unwrap();

        match input.trim().to_lowercase().as_str() {
            "exit" => break,
            "exit 0" => break,
            _ => println!("{}: command not found", input.trim()),
        };
    }
}
