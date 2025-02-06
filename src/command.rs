use crate::{os::StandardIO, parser::parse_input};

pub struct ShellCommand {
    name: String,
    arguments: Vec<String>,
    io_out: StandardIO,
    io_err: StandardIO,
}

enum Token {
    Literal(String),
    Operator(String),
}

#[derive(Clone)]
enum Stage {
    Name,
    Args,
    Redirect(String),
}

enum Redirect {
    None,
    Out(StandardIO),
    Err(StandardIO),
}

impl From<String> for Token {
    fn from(value: String) -> Self {
        if value.contains('>') {
            Self::Operator(value)
        } else {
            Self::Literal(value)
        }
    }
}

impl ShellCommand {
    pub fn with_input(input: &str) -> Option<Self> {
        let input = input.trim();
        let tokens = parse_input(input);
        if tokens.is_empty() {
            return None;
        }
        let mut name = String::new();
        let mut arguments = Vec::new();
        let mut io_out = StandardIO::Default;
        let mut io_err = StandardIO::Default;

        let mut stage = Stage::Name;
        let tokens = tokens.into_iter().map(Token::from).collect::<Vec<_>>();
        for token in tokens {
            match (stage.clone(), token) {
                (Stage::Name, Token::Literal(value)) => {
                    name = value;
                    stage = Stage::Args;
                }
                (Stage::Args, Token::Literal(value)) => {
                    arguments.push(value);
                }
                (Stage::Args, Token::Operator(value)) => {
                    stage = Stage::Redirect(value);
                }
                (Stage::Redirect(operator), Token::Literal(value)) => {
                    match Self::parse_redirect(&operator, &value) {
                        Redirect::Out(value) => io_out = value,
                        Redirect::Err(value) => io_err = value,
                        _ => {
                            // no op
                        }
                    }
                    // don't process next tokens...
                    break;
                }
                _ => {
                    // Invalid input...
                    return None;
                }
            }
        }
        if name.is_empty() {
            return None;
        }
        Some(Self {
            name,
            arguments,
            io_out,
            io_err,
        })
    }

    fn parse_redirect(operator: &str, value: &str) -> Redirect {
        match operator {
            ">" | "1>" => {
                // override stdout
                Redirect::Out(StandardIO::File {
                    path: value.to_string(),
                    append: false,
                })
            }
            "2>" => {
                // override stderr
                Redirect::Err(StandardIO::File {
                    path: value.to_string(),
                    append: false,
                })
            }
            _ => {
                // not supported
                Redirect::None
            }
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn args(&self) -> &[String] {
        &self.arguments
    }

    pub fn io_out(&self) -> &StandardIO {
        &self.io_out
    }

    pub fn io_err(&self) -> &StandardIO {
        &self.io_err
    }
}
