mod builtins;
mod os;
mod parser;

use std::io::{self, Write};

use builtins::{dispatch_builtin, Builtin};
use os::{find_file, get_search_path};
use parser::parse_input;

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
    let tokens = parse_input(input);
    let Some(command) = tokens.first() else {
        return;
    };
    let args = &tokens[1..];

    if let Ok(builtin) = Builtin::try_from(command.as_str()) {
        dispatch_builtin(builtin, &args);
        return;
    };

    if exec(command, &args) {
        return;
    }

    invalid_input(input);
}

fn invalid_input(input: &str) {
    println!("{}: command not found", input);
}

fn exec<T: AsRef<str>>(program: &str, args: &[T]) -> bool {
    let Ok(path_list) = get_search_path() else {
        return false;
    };
    if find_file(program, &path_list).is_empty() {
        return false;
    }
    let mut process = std::process::Command::new(program);
    if !args.is_empty() {
        let args = args.iter().map(|s| s.as_ref()).collect::<Vec<_>>();
        process.args(args);
    }
    let Ok(mut child) = process.spawn() else {
        return false;
    };
    child.wait().is_ok()
}
