use std::{env, fmt::Display, io::Write};

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

impl Builtin {
    pub fn all() -> Vec<Self> {
        use Builtin::*;
        vec![Cd, Echo, Exit, Type, Pwd]
    }
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

impl Display for Builtin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            Builtin::Cd => CMD_CD,
            Builtin::Echo => CMD_ECHO,
            Builtin::Exit => CMD_EXIT,
            Builtin::Type => CMD_TYPE,
            Builtin::Pwd => CMD_PWD,
        };
        write!(f, "{}", val)
    }
}

pub fn exec_builtin(builtin: Builtin, command: &ShellCommand) {
    let Ok(mut out) = command.io_out().try_stdout_write() else {
        return;
    };
    let Ok(mut err) = command.io_err().try_stderr_write() else {
        return;
    };

    let args = command.args();

    match builtin {
        Builtin::Cd => cmd_cd(args),
        Builtin::Echo => cmd_echo(args, &mut out),
        Builtin::Exit => cmd_exit(args),
        Builtin::Type => cmd_type(args, &mut out),
        Builtin::Pwd => cmd_pwd(&mut out),
    }

    _ = out.flush();
    _ = err.flush();
}

fn cmd_cd(args: &[String]) {
    let Some(arg) = args.first() else {
        return;
    };

    let path = if let Some(rest) = arg.strip_prefix("~") {
        let Ok(home) = env::var("HOME") else {
            return;
        };
        format!("{}{}", home, rest)
    } else {
        arg.to_string()
    };

    if change_working_directory(&path).is_err() {
        println!("cd: {}: No such file or directory", arg);
    }
}

fn cmd_echo(args: &[String], out: &mut impl Write) {
    let output = args.join(" ");
    _ = writeln!(out, "{}", output);
}

fn cmd_exit(args: &[String]) {
    let code = args
        .first()
        .map(|s| s.parse::<i32>())
        .and_then(|x| x.ok())
        .unwrap_or_default();
    std::process::exit(code);
}

fn cmd_type(args: &[String], out: &mut impl Write) {
    let Some(args) = args.first() else {
        return;
    };

    if Builtin::try_from(args.as_ref()).is_ok() {
        _ = writeln!(out, "{} is a shell builtin", args);
        return;
    }

    if let Ok(path_list) = get_search_path() {
        if let Some(path) = find_file(args, &path_list).first() {
            _ = writeln!(
                out,
                "{} is {}",
                args,
                path.as_os_str().to_str().unwrap_or_default()
            );
            return;
        }
    }

    _ = writeln!(out, "{}: not found", args);
}

fn cmd_pwd(out: &mut impl Write) {
    let Ok(path) = get_working_directory() else {
        return;
    };
    _ = writeln!(out, "{}", path.display());
}
