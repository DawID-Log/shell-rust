#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    
    print!("$ ");
    while stdin.read_line(&mut input).is_ok() {
        println!("{}: command not found", input.strip_suffix("\n").unwrap());
        input.clear();
        print!("$ ");
        io::stdout().flush().unwrap();
    }
}
