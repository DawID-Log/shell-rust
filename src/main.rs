#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;

fn not_found(command: &str) {
    println!("{}: command not found", command);
}

fn tokenize(input: &str) -> Vec<&str> {
    input.split(' ').collect()
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let command = input.trim();
        let token = tokenize(command);
        match token[..] {
            ["exit", code] => process::exit(code.parse::<i32>().unwrap()),
            ["echo", ..] => println!("{}", token[1..].join(" ")),
            _ => not_found(command),
        };
    }
}
