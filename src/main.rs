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
        check_if_exit(&input);
        println!("{}: command not found", input.trim());
    }
}

fn check_if_exit(command: &str) {
    if command.starts_with("exit") {
        std::process::exit(0)
    }
}
