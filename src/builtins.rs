use std::env;

use crate::{
    command::ShellCommand,
    os::{change_working_directory, find_file, get_search_path, get_working_directory},
};

const CMD_CD: &str = "cd";
const CMD_ECHO: &str = "echo";
const CMD_EXIT: &str = "exit";
const CMD_TYPE: &str = "type";
const CMD_PWD: &str = "pwd";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Builtin {
    Cd,
    Echo,
    Exit,
    Type,
    Pwd,
}

impl TryFrom<&str> for Builtin {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            CMD_CD => Ok(Builtin::Cd),
            CMD_ECHO => Ok(Builtin::Echo),
            CMD_EXIT => Ok(Builtin::Exit),
            CMD_TYPE => Ok(Builtin::Type),
            CMD_PWD => Ok(Builtin::Pwd),
            _ => Err(format!("Unknown builtin {}", value)),
        }
    }
}

pub fn exec_builtin(builtin: Builtin, command: &ShellCommand) {
    match builtin {
        Builtin::Cd => cmd_cd(command),
        Builtin::Echo => cmd_echo(command),
        Builtin::Exit => cmd_exit(command),
        Builtin::Type => cmd_type(command),
        Builtin::Pwd => cmd_pwd(command),
    }
}

fn cmd_cd(command: &ShellCommand) {
    let Some(arg) = command.args().first() else {
        return;
    };

    let path = if arg.starts_with("~") {
        let Ok(home) = env::var("HOME") else {
            return;
        };
        format!("{}{}", home, &arg[1..])
    } else {
        arg.to_string()
    };

    if change_working_directory(&path).is_err() {
        println!("cd: {}: No such file or directory", arg);
    }
}

fn cmd_echo(command: &ShellCommand) {
    let output = command.args().join(" ");
    println!("{}", output);
}

fn cmd_exit(_command: &ShellCommand) {
    std::process::exit(0);
}

fn cmd_type(command: &ShellCommand) {
    let Some(args) = command.args().first() else {
        return;
    };

    if Builtin::try_from(args.as_ref()).is_ok() {
        println!("{} is a shell builtin", args);
        return;
    }

    if let Ok(path_list) = get_search_path() {
        if let Some(path) = find_file(args, &path_list).first() {
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

fn cmd_pwd(command: &ShellCommand) {
    let Ok(path) = get_working_directory() else {
        return;
    };
    println!("{}", path.display());
}
