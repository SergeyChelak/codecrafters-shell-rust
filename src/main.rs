mod builtins;
mod os;

use std::{
    io::{self, Write},
    iter,
};

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
        let args = parse_args(args);
        process.args(&args);
    }
    let Ok(mut child) = process.spawn() else {
        return false;
    };
    child.wait().is_ok()
}

fn parse_args(args: &str) -> Vec<String> {
    let mut is_enclosing = false;
    let mut acc: Vec<char> = Vec::new();
    let mut tokens: Vec<String> = Vec::new();
    for ch in args.chars().chain(iter::once('\0')) {
        if ch == '\'' {
            is_enclosing = !is_enclosing;
            continue;
        }
        if ch == ' ' && !is_enclosing || ch == '\0' {
            let token = acc.iter().collect::<String>();
            // TODO: trim?
            tokens.push(token);
            acc.clear();
            continue;
        }
        acc.push(ch);
    }
    tokens
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_args_test_1() {
        let args = "'shell hello'";

        let res = parse_args(args);
        assert!(res.len() == 1);
        assert_eq!(res[0], "shell hello")
    }

    #[test]
    fn parse_args_test_2() {
        let args = "'/tmp/file name' '/tmp/file name with spaces'";

        let res = parse_args(args);
        assert!(res.len() == 2);
        assert_eq!(res[0], "/tmp/file name");
        assert_eq!(res[1], "/tmp/file name with spaces");
    }
}
