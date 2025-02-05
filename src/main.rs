use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    let registry = make_command_registry();
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        dispatch(&input, &registry);
    }
}

type Command = dyn Fn(&str);
type CommandRegistry = HashMap<String, &'static Command>;

const CMD_ECHO: &str = "echo";
const CMD_EXIT: &str = "exit";
const CMD_TYPE: &str = "type";
const CMD_PWD: &str = "pwd";

fn make_builtin_list() -> Vec<&'static str> {
    vec![CMD_ECHO, CMD_EXIT, CMD_TYPE]
}

fn make_command_registry() -> CommandRegistry {
    let commands: Vec<(&'static str, &'static Command)> = vec![
        (CMD_ECHO, &echo),
        (CMD_EXIT, &exit),
        (CMD_TYPE, &type_builtin),
        (CMD_PWD, &pwd),
    ];
    let mut registry = HashMap::new();
    for (name, func) in commands {
        registry.insert(name.to_string(), func);
    }
    registry
}

fn dispatch(input: &str, registry: &CommandRegistry) {
    let input = input.trim();
    let (command, args) = input.split_once(' ').unwrap_or((input, ""));
    if let Some(func) = registry.get(command) {
        func(args);
        return;
    };

    if try_exec(command, args) {
        return;
    }

    invalid_input(input);
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
    let builtins = make_builtin_list();
    if builtins.contains(&args) {
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

fn pwd(_args: &str) {
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
