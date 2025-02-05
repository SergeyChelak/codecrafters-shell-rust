use std::fmt::Display;

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

impl Display for Builtin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            Builtin::Echo => CMD_ECHO,
            Builtin::Exit => CMD_EXIT,
            Builtin::Type => CMD_TYPE,
            Builtin::Pwd => CMD_PWD,
        };
        write!(f, "{}", val)
    }
}
