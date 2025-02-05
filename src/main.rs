mod builtins;
mod os;

use std::io::{self, Write};

use builtins::{dispatch_builtin, Builtin};
use os::{find_file, get_search_path};

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

fn dispatch(input: &str) {
    let input = input.trim();
    let (command, args) = input.split_once(' ').unwrap_or((input, ""));

    if let Ok(builtin) = Builtin::try_from(command) {
        dispatch_builtin(builtin, args);
        return;
    };

    if exec(command, args) {
        return;
    }

    invalid_input(input);
}

fn invalid_input(input: &str) {
    println!("{}: command not found", input);
}

fn exec(program: &str, args: &str) -> bool {
    let Ok(path_list) = get_search_path() else {
        return false;
    };
    if find_file(program, &path_list).is_empty() {
        return false;
    }
    let mut process = std::process::Command::new(program);
    if !args.is_empty() {
        process.args(&[args]);
    }
    let Ok(mut child) = process.spawn() else {
        return false;
    };
    child.wait().is_ok()
}
