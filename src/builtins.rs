use crate::os::{find_file, get_search_path, get_working_path};

const CMD_ECHO: &str = "echo";
const CMD_EXIT: &str = "exit";
const CMD_TYPE: &str = "type";
const CMD_PWD: &str = "pwd";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Builtin {
    Echo,
    Exit,
    Type,
    Pwd,
}

impl TryFrom<&str> for Builtin {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            CMD_ECHO => Ok(Builtin::Echo),
            CMD_EXIT => Ok(Builtin::Exit),
            CMD_TYPE => Ok(Builtin::Type),
            CMD_PWD => Ok(Builtin::Pwd),
            _ => Err(format!("Unknown builtin {}", value)),
        }
    }
}

pub fn dispatch_builtin(command: Builtin, args: &str) {
    match command {
        Builtin::Echo => echo(args),
        Builtin::Exit => exit(args),
        Builtin::Type => type_builtin(args),
        Builtin::Pwd => pwd(),
    }
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

fn pwd() {
    let Ok(path) = get_working_path() else {
        return;
    };
    println!("{}", path.display());
}
