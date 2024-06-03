#[allow(unused_imports)]
use std::io::{self, Write};

fn handle_command(input: &str) -> bool {
    let path_env = std::env::var("PATH").unwrap();
    let builtins = ["exit", "echo", "type", "cd"];
    let cmds = input.split_whitespace().collect::<Vec<&str>>();
    let cmd = cmds[0];
    let args = &cmds[1..];
    if cmd.is_empty() {
        return false;
    }

    if let Some(path) = find_exe(cmd){
        std::process::Command::new(path)
            .args(args)
            .status()
            .expect("failed to execute process");
        return false;
    }

    match cmd {
        "help" => {
            println!("exit [n]: The exit utility shall cause the shell to exit from its current execution environment with the exit status specified by the unsigned decimal integer n. If the current execution environment is a subshell environment, the shell shall exit from the subshell environment with the specified exit status and continue in the environment from which that subshell environment was invoked; otherwise, the shell utility shall terminate with the specified exit status. If n is specified, but its value is not between 0 and 255 inclusively, the exit status is undefined.");
            println!("\necho [string...]: The echo utility writes its arguments to standard output, followed by a <newline>. If there are no arguments, only the <newline> is written.");
            println!("\ntype <command names>: The type command is used to describe how its argument would be translated if used as commands. It is also used to find out whether it is built-in or external binary file ");
            println!("\ncd <directory>: The type command is used to describe how its argument would be translated if used as commands. It is also used to find out whether it is built-in or external binary file");
            println!("\npwd: The pwd Linux command prints the current working directory path, starting from the root (/). Use the pwd command to find your way in the Linux file system structure maze or to pass the working directory in a Bash script.");
        },
        "exit" => {
            if cmds.len() == 1 || (cmds.len() > 1 && args[0] == "0") {
                return true;
            } else {
                println!("{}: command not found", cmd.trim());
            }
        },
        "echo" => println!("{}", args[0..].join(" ")),
        "type" => {
            let arg = args[0];
            if cmds.len() != 2 {
                println!("type: expected 1 argument, got {}", cmds.len() - 1);
                return false;
            }
            if builtins.contains(&arg) {
                println!("{} is a shell builtin", arg);
            } else {
                let split = &mut path_env.split(':');
                if let Some(path) =
                    split.find(|path| std::fs::metadata(format!("{}/{}", path, args[0])).is_ok())
                {
                    println!("{arg} is {path}/{arg}");
                } else {
                    println!("{arg} not found");
                }
            }
        },
        "cd" => {
            let mut arg = args[0].to_owned();
            if arg == "~" {
                arg = std::env::var("HOME").unwrap();
            }else if !arg.starts_with("/") {
                arg = format!("{}/{}", std::env::current_dir().unwrap().to_str().unwrap(), arg);
            }

            if std::env::set_current_dir(std::path::Path::new(&arg)).is_err() {
                println!("cd: {}: No such file or directory", &arg);
            }
        },
        _ => println!("{}: command not found", cmd.trim()),
    }
    return false
}

fn find_exe(name: &str) -> Option<std::path::PathBuf> {
    if let Ok(paths) = std::env::var("PATH") {
        for path in std::env::split_paths(&paths) {
            let exe_path = path.join(name);
            if exe_path.is_file() {
                return Some(exe_path);
            }
        }
    }
    return None;
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
