mod builtins;
mod command;
mod os;
mod parser;

use std::{
    fs::OpenOptions,
    io::{self, Write},
    process::Stdio,
};

use builtins::{exec_builtin, Builtin};
use command::{ShellCommand, StandardIO};
use os::{find_file, get_search_path};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
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
        // TODO: fix it
        exec_builtin(builtin, &command);
        return;
    };

    if exec(&command) {
        return;
    }

    invalid_input(input);
}

fn invalid_input(input: &str) {
    println!("{}: command not found", input);
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

    if let Ok(stdout) = stdout_from(cmd.io_out()) {
        command.stdout(stdout);
    }

    if let Ok(stderr) = stderr_from(cmd.io_err()) {
        command.stderr(stderr);
    }

    let Ok(mut child) = command.spawn() else {
        return false;
    };
    child.wait().is_ok()
}

fn stdout_from(stdio: &StandardIO) -> io::Result<Stdio> {
    match stdio {
        StandardIO::Default => Ok(io::stdout().into()),
        StandardIO::File { path, append } => make_stdio(&path, *append),
    }
}

fn stderr_from(stdio: &StandardIO) -> io::Result<Stdio> {
    match stdio {
        StandardIO::Default => Ok(io::stderr().into()),
        StandardIO::File { path, append } => make_stdio(&path, *append),
    }
}

fn make_stdio(path: &str, append: bool) -> io::Result<Stdio> {
    let file = OpenOptions::new()
        .append(append)
        .write(true)
        .create(true)
        .open(path)?;
    Ok(Stdio::from(file))
}
