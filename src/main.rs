#[allow(unused_imports)]
use std::io::{self, Write};

fn handle_command(input: &str) -> bool {
    let path_env = std::env::var("PATH").unwrap();
    let builtins = ["exit", "echo", "type"];
    let argv = input.split_whitespace().collect::<Vec<&str>>();
    if argv.is_empty() {
        return false;
    }

    println!("argv: [{}]", argv[0..].join(" - "));


    match argv[0] {
        "exit" => {
            if argv.len() == 1 || (argv.len() > 1 && argv[1] == "0") {
                return true;
            } else {
                println!("{}: command not found", argv[0]);
            }
        },
        "echo" => println!("{}", argv[1..].join(" ")),
        "type" => {
            if argv.len() != 2 {
                println!("type: expected 1 argument, got {}", argv.len() - 1);
                return false;
            }
            let cmd = argv[1];
            if builtins.contains(&cmd) {
                println!("{} is a shell builtin", cmd);
            } else {
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
        _ => println!("{}: command not found", argv[0].trim()),
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
