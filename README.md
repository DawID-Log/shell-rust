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
