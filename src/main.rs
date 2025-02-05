#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        dispatch(&input);
    }
}

fn dispatch(command: &str) {
    let command = command.trim();

    if try_echo(command) {
        return;
    }

    if try_exit(command) {
        return;
    }

    println!("{}: command not found", command);
}

const CMD_ECHO: &str = "echo";
const CMD_EXIT: &str = "exit";

fn try_echo(command: &str) -> bool {
    if !command.starts_with(CMD_ECHO) {
        return false;
    }
    let output = &command[CMD_ECHO.len() + 1..];
    println!("{}", output);
    true
}

fn try_exit(command: &str) -> bool {
    if command.starts_with(CMD_EXIT) {
        std::process::exit(0);
    }
    false
}
