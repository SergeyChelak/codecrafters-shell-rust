mod builtins;
mod command;
mod os;
mod parser;

use std::io::{self, Write};

use builtins::{exec_builtin, Builtin};
use command::ShellCommand;
use os::{find_file, get_search_path};

fn main() {
    let stdin = io::stdin();
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        process_input(&input);
    }
}

fn process_input(input: &str) {
    let Some(command) = ShellCommand::with_input(input) else {
        // TODO: handle invalid commands
        return;
    };

    if let Ok(builtin) = Builtin::try_from(command.name()) {
        exec_builtin(builtin, &command);
        return;
    };

    if exec(&command) {
        return;
    }

    invalid_input(input);
}

fn invalid_input(input: &str) {
    println!("{}: command not found", input.trim());
}

fn exec(cmd: &ShellCommand) -> bool {
    let Ok(path_list) = get_search_path() else {
        return false;
    };
    if find_file(cmd.name(), &path_list).is_empty() {
        return false;
    }
    let mut command = std::process::Command::new(cmd.name());
    let args = cmd.args();
    if !args.is_empty() {
        command.args(args);
    }

    if let Ok(stdout) = cmd.io_out().try_stdout() {
        command.stdout(stdout);
    }

    if let Ok(stderr) = cmd.io_err().try_stderr() {
        command.stderr(stderr);
    }

    let Ok(mut child) = command.spawn() else {
        return false;
    };
    child.wait().is_ok()
}
