use rustyline::completion::{Completer, Pair};
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
    let helper = EditorHelper {
        completer: BuiltinCompleter::new(),
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
