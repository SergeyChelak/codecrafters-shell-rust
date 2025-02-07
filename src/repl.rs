use rustyline::completion::{Completer, FilenameCompleter, Pair};
use rustyline::{
    Completer, CompletionType, Config, Editor, Helper, Highlighter, Hinter, Validator,
};

use crate::builtins::Builtin;

#[derive(Helper, Completer, Hinter, Validator, Highlighter)]
struct EditorHelper<C: Completer> {
    #[rustyline(Completer)]
    completer: C,
}

pub fn repl(handler: impl Fn(&str)) {
    let config = Config::builder()
        .completion_type(CompletionType::Circular)
        // .edit_mode(EditMode::Emacs)
        .build();
    let mut autocomplete = ShellCompleter::new();
    autocomplete.add(BuiltinCompleter::new());
    autocomplete.add(FilenameCompleter::new());
    let helper = EditorHelper {
        completer: autocomplete,
    };

    let Ok(mut editor) = Editor::with_config(config) else {
        // process error
        return;
    };
    editor.set_helper(Some(helper));

    loop {
        let Ok(input) = editor.readline("$ ") else {
            break;
        };
        handler(&input);
    }
}

struct ShellCompleter {
    completers: Vec<Box<dyn Completer<Candidate = Pair>>>,
}

impl ShellCompleter {
    pub fn new() -> Self {
        Self {
            completers: Vec::new(),
        }
    }

    pub fn add(&mut self, completer: impl Completer<Candidate = Pair> + 'static) {
        let wrap: Box<dyn Completer<Candidate = Pair>> = Box::new(completer);
        self.completers.push(wrap);
    }
}

impl Completer for ShellCompleter {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        for completer in self.completers.iter() {
            let Ok(res) = completer.complete(line, pos, ctx) else {
                continue;
            };
            if !res.1.is_empty() {
                return Ok(res);
            }
        }
        Ok((0, Vec::with_capacity(0)))
    }
}

struct BuiltinCompleter {
    //
}

impl BuiltinCompleter {
    fn new() -> Self {
        Self {}
    }
}

impl Completer for BuiltinCompleter {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        let candidates = Builtin::all()
            .iter()
            .map(|x| format!("{x}"))
            .filter(|s| s.starts_with(line))
            .map(|x| Pair {
                display: x.clone(),
                replacement: format!("{} ", &x[pos..]),
            })
            .collect::<Vec<_>>();

        Ok((pos, candidates))
    }
}
