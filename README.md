# Passing the first stage

The entry point for your `shell` implementation is in `src/main.rs`.

# Stage 2

Note: This section is for stages 2 and beyond.

1. Ensure you have `cargo (1.70)` installed locally
1. Run `./your_shell.sh` to run your program, which is implemented in
   `src/main.rs`. This command compiles your Rust project, so it might be slow
   the first time you run it. Subsequent runs will be fast.
1. Commit your changes and run `git push origin master` to submit your solution
   to CodeCrafters. Test output will be streamed to your terminal.

I used WSL and I chose to use “Ubuntu”:
```sh
bash
wsl --install -d Ubuntu
```
To choose the distro:
```sh
wsl --list --online
```

# Stage 3
To start the shell, run:
```sh
bash
exec cargo run --quiet --release --target-dir=/tmp/codecrafters-shell-target
```

# What is possible to do
Running the help command results in a detailed description of the various commands available:
## exit [n]
"The exit utility shall cause the shell to exit from its current execution environment with the exit status specified by the unsigned decimal integer n. If the current execution environment is a subshell environment, the shell shall exit from the subshell environment with the specified exit status and continue in the environment from which that subshell environment was invoked; otherwise, the shell utility shall terminate with the specified exit status. If n is specified, but its value is not between 0 and 255 inclusively, the exit status is undefined."
## echo [string...]
"The echo utility writes its arguments to standard output, followed by a <_newline_>. If there are no arguments, only the <_newline_> is written."
## type <_command names_>
"The type command is used to describe how its argument would be translated if used as commands. It is also used to find out whether it is built-in or external binary file"
## cd <_directory_>
The type command is used to describe how its argument would be translated if used as commands. It is also used to find out whether it is built-in or external binary file"
## pwd
The pwd Linux command prints the current working directory path, starting from the root (/). Use the pwd command to find your way in the Linux file system structure maze or to pass the working directory in a Bash script."