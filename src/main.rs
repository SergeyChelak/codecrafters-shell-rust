use std::collections::HashMap;
#[allow(unused_imports)]
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

fn make_builtin_list() -> Vec<&'static str> {
    vec![CMD_ECHO, CMD_EXIT, CMD_TYPE]
}

fn make_command_registry() -> CommandRegistry {
    let commands: Vec<(&'static str, &'static Command)> = vec![
        (CMD_ECHO, &echo),
        (CMD_EXIT, &exit),
        (CMD_TYPE, &type_builtin),
    ];
    let mut registry = HashMap::new();
    for (name, func) in commands {
        registry.insert(name.to_string(), func);
    }
    registry
}

fn dispatch(input: &str, registry: &CommandRegistry) {
    let Some((command, args)) = input.trim().split_once(' ') else {
        invalid_input(input);
        return;
    };

    let Some(func) = registry.get(command) else {
        invalid_input(input);
        return;
    };

    func(args);
}

fn invalid_input(input: &str) {
    println!("{}: command not found", input.trim());
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

fn get_search_path() -> Result<Vec<String>, std::env::VarError> {
    let var = std::env::var("PATH")?;
    Ok(var.split(":").map(|x| x.to_string()).collect::<Vec<_>>())
}
