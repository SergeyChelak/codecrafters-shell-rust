mod builtins;

use std::io::{self, Write};

use builtins::Builtin;

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

    if try_exec(command, args) {
        return;
    }

    invalid_input(input);
}

fn dispatch_builtin(command: Builtin, args: &str) {
    match command {
        Builtin::Echo => echo(args),
        Builtin::Exit => exit(args),
        Builtin::Type => type_builtin(args),
        Builtin::Pwd => pwd(),
    }
}

fn invalid_input(input: &str) {
    println!("{}: command not found", input);
}

fn echo(args: &str) {
    println!("{}", args);
}

fn exit(_args: &str) {
    std::process::exit(0);
}

fn type_builtin(args: &str) {
    if Builtin::try_from(args).is_ok() {
        println!("{} is a shell builtin", args);
        return;
    }

    if let Ok(path_list) = get_search_path() {
        for path in path_list
            .iter()
            .map(|s| std::path::Path::new(s))
            .map(|p| p.join(args))
        {
            if !path.exists() {
                continue;
            }
            println!(
                "{} is {}",
                args,
                path.as_os_str().to_str().unwrap_or_default()
            );
            return;
        }
    }

    println!("{}: not found", args);
}

fn pwd() {
    let Ok(path) = std::env::current_dir() else {
        return;
    };
    println!("{}", path.display());
}

fn try_exec(program: &str, args: &str) -> bool {
    let Ok(path_list) = get_search_path() else {
        return false;
    };
    if path_list
        .iter()
        .map(|s| std::path::Path::new(s))
        .map(|p| p.join(program))
        .any(|p| p.exists())
    {
        return exec(program, args);
    }
    false
}

fn exec(program: &str, args: &str) -> bool {
    let Ok(mut child) = std::process::Command::new(program).args(&[args]).spawn() else {
        return false;
    };
    child.wait().is_ok()
}

fn get_search_path() -> Result<Vec<String>, std::env::VarError> {
    let var = std::env::var("PATH")?;
    Ok(var.split(":").map(|x| x.to_string()).collect::<Vec<_>>())
}
