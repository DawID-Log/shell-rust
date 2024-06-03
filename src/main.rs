#[allow(unused_imports)]
use std::io::{self, Write};

fn handle_command(input: &str) -> bool {
    let path_env = std::env::var("PATH").unwrap();
    let builtins = ["exit", "echo", "type"];
    let argv = input.split_whitespace().collect::<Vec<&str>>();
    if argv.is_empty() {
        return true;
    }
    let parts: Vec<&str> = input.split_whitespace().collect();

    match parts[0] {
        "exit" => {
            if parts.len() == 1 || (parts.len() > 1 && parts[1] == "0") {
                return false;
            } else {
                println!("{}: command not found", input);
            }
        },
        "echo" => println!("{}", &input[5..]),
        "type" => {
            if argv.len() != 2 {
                println!("type: expected 1 argument, got {}", argv.len() - 1);
                return false;
            }
            let cmd = argv[1];
            if builtins.contains(&cmd) {
                println!("{} is a shell builtin", cmd);
            } else {
                println!("{} not found", cmd);
                let split = &mut path_env.split(':');
                if let Some(path) =
                    split.find(|path| std::fs::metadata(format!("{}/{}", path, cmd)).is_ok())
                {
                    println!("{cmd} is {path}/{cmd}");
                } else {
                    println!("{cmd} not found");
                }
            }
        },
        _ => println!("{}: command not found", input.trim()),
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
