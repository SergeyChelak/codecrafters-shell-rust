use std::iter;

pub fn parse_input(input: &str) -> Vec<String> {
    let mut parser = Parser::new(input);
    parser.parse();
    parser.tokens
}

enum Enclose {
    Active(char),
    None,
}

impl Enclose {
    fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    fn is_enclosing(&self) -> bool {
        !self.is_none()
    }

    fn is_enclosed_with(&self, ch: char) -> bool {
        match self {
            Self::Active(enc) => ch == *enc,
            Self::None => false,
        }
    }
}

struct Parser<'a> {
    acc: Vec<char>,
    tokens: Vec<String>,
    enclose: Enclose,
    is_escaping: bool,
    input: &'a str,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Parser {
            acc: Vec::new(),
            tokens: Vec::new(),
            enclose: Enclose::None,
            is_escaping: false,
            input,
        }
    }

    pub fn parse(&mut self) {
        let next = self.input.chars().chain(['\0', '\0']).skip(1);
        for (ch, next) in self.input.chars().chain(iter::once('\0')).zip(next) {
            if self.try_process_escaping(ch) {
                continue;
            }
            if self.try_update_enclose(ch) {
                continue;
            }
            if self.try_escaping(ch, next) {
                continue;
            }
            if self.try_flush_token(ch) {
                continue;
            }
            self.acc.push(ch);
        }
    }

    fn try_process_escaping(&mut self, ch: char) -> bool {
        if !self.is_escaping {
            return false;
        }
        self.acc.push(ch);
        self.is_escaping = false;
        true
    }

    fn try_escaping(&mut self, ch: char, next: char) -> bool {
        if self.enclose.is_enclosed_with('\'') || self.is_escaping || ch != '\\' {
            return false;
        }

        if self.enclose.is_enclosed_with('\"') && !matches!(next, '$' | '\\' | '\"' | '\n') {
            return false;
        }

        self.is_escaping = true;
        true
    }

    fn try_update_enclose(&mut self, ch: char) -> bool {
        if !matches!(ch, '\'' | '\"') {
            return false;
        }
        if self.enclose.is_none() {
            self.enclose = Enclose::Active(ch);
            return true;
        }
        if self.enclose.is_enclosed_with(ch) {
            self.enclose = Enclose::None;
            return true;
        }
        false
    }

    fn try_flush_token(&mut self, ch: char) -> bool {
        if ch == ' ' && !self.enclose.is_enclosing() || ch == '\0' {
            self.flush_token();
            return true;
        }
        false
    }

    fn flush_token(&mut self) {
        if self.acc.is_empty() {
            return;
        }
        let token = self.acc.iter().collect::<String>();
        // TODO: trim...
        self.tokens.push(token);
        self.acc.clear();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_args_test_empty() {
        let args = "";

        let res = parse_input(args);
        assert!(res.is_empty());
    }

    #[test]
    fn parse_args_test_single_quote_1() {
        let args = "'shell hello'";

        let res = parse_input(args);
        assert!(res.len() == 1);
        assert_eq!(res[0], "shell hello")
    }

    #[test]
    fn parse_args_test_single_quote_2() {
        let args = "'/tmp/file name' '/tmp/file name with spaces'";

        let res = parse_input(args);
        assert!(res.len() == 2);
        assert_eq!(res[0], "/tmp/file name");
        assert_eq!(res[1], "/tmp/file name with spaces");
    }

    #[test]
    fn parse_args_test_double_quote_2() {
        let args = "\"quz  hello\"  \"bar\"";

        let res = parse_input(args);
        assert!(res.len() == 2);
        assert_eq!(res[0], "quz  hello");
        assert_eq!(res[1], "bar")
    }

    #[test]
    fn parse_args_test_double_quote_enclose_single_quotes() {
        let args = "\"/tmp/file name\" \"/tmp/'file name' with spaces\"";
        let res = parse_input(args);
        assert!(res.len() == 2);
        assert_eq!(res[0], "/tmp/file name");
        assert_eq!(res[1], "/tmp/'file name' with spaces");
    }

    #[test]
    fn parse_args_test_mixed_quotes_3() {
        let args = "\"bar\"  \"shell's\"  \"foo\"";

        let res = parse_input(args);
        assert!(res.len() == 3);
        assert_eq!(res[0], "bar");
        assert_eq!(res[1], "shell's");
        assert_eq!(res[2], "foo");
    }

    #[test]
    fn parse_args_test_enclosed_backslash() {
        let args = "'/tmp/baz/\"f\\91\"'";

        let res = parse_input(args);
        assert!(res.len() == 1);
        assert_eq!(res[0], "/tmp/baz/\"f\\91\"");
    }
}
