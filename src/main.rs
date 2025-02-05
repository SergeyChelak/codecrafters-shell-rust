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

    let args = parse_args(args);

    if let Ok(builtin) = Builtin::try_from(command) {
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

enum Enclose {
    Active(char),
    None,
}

impl Enclose {
    fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    fn is_enclosing(&self) -> bool {
        !self.is_none()
    }

    fn is_enclosed_with(&self, ch: char) -> bool {
        match self {
            Self::Active(enc) => ch == *enc,
            Self::None => false,
        }
    }
}

fn is_enclose_char(ch: char) -> bool {
    match ch {
        '\'' | '\"' => true,
        _ => false,
    }
}

fn parse_args(args: &str) -> Vec<String> {
    let mut acc: Vec<char> = Vec::new();
    let mut tokens: Vec<String> = Vec::new();

    let mut enclose = Enclose::None;
    for ch in args.chars().chain(iter::once('\0')) {
        if is_enclose_char(ch) {
            if enclose.is_none() {
                enclose = Enclose::Active(ch);
                continue;
            }
            if enclose.is_enclosed_with(ch) {
                enclose = Enclose::None;
                continue;
            }
        }
        if ch == ' ' && !enclose.is_enclosing() || ch == '\0' {
            if !acc.is_empty() {
                let token = acc.iter().collect::<String>();
                // TODO: trim?
                tokens.push(token);
                acc.clear();
            }
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
    fn parse_args_test_empty() {
        let args = "";

        let res = parse_args(args);
        assert!(res.is_empty());
    }

    #[test]
    fn parse_args_test_single_quote_1() {
        let args = "'shell hello'";

        let res = parse_args(args);
        assert!(res.len() == 1);
        assert_eq!(res[0], "shell hello")
    }

    #[test]
    fn parse_args_test_single_quote_2() {
        let args = "'/tmp/file name' '/tmp/file name with spaces'";

        let res = parse_args(args);
        assert!(res.len() == 2);
        assert_eq!(res[0], "/tmp/file name");
        assert_eq!(res[1], "/tmp/file name with spaces");
    }

    #[test]
    fn parse_args_test_double_quote_2() {
        let args = "\"quz  hello\"  \"bar\"";

        let res = parse_args(args);
        assert!(res.len() == 2);
        assert_eq!(res[0], "quz  hello");
        assert_eq!(res[1], "bar")
    }

    #[test]
    fn parse_args_test_double_quote_enclose_single_quotes() {
        let args = "\"/tmp/file name\" \"/tmp/'file name' with spaces\"";
        let res = parse_args(args);
        assert!(res.len() == 2);
        assert_eq!(res[0], "/tmp/file name");
        assert_eq!(res[1], "/tmp/'file name' with spaces");
    }

    #[test]
    fn parse_args_test_mixed_quotes_3() {
        let args = "\"bar\"  \"shell's\"  \"foo\"";

        let res = parse_args(args);
        assert!(res.len() == 3);
        assert_eq!(res[0], "bar");
        assert_eq!(res[1], "shell's");
        assert_eq!(res[2], "foo");
    }
}
