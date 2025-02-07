use rustyline::completion::{Completer, Pair};
use rustyline::{
    Completer, CompletionType, Config, Editor, Helper, Highlighter, Hinter, Validator,
};

use crate::builtins::Builtin;
use crate::os::{get_search_path, get_working_directory};

#[derive(Helper, Completer, Hinter, Validator, Highlighter)]
struct EditorHelper<C: Completer> {
    #[rustyline(Completer)]
    completer: C,
}

pub fn repl(handler: impl Fn(&str)) {
    let config = Config::builder()
        .completion_type(CompletionType::List)
        .build();
    let mut autocomplete = ShellCompleter::new();
    autocomplete.add(BuiltinCompleter);
    if let Ok(path) = get_working_directory().map(|path| path.display().to_string()) {
        autocomplete.add(FilesystemCompleter { path });
    }
    if let Ok(path_list) = get_search_path() {
        path_list.into_iter().for_each(|path| {
            autocomplete.add(FilesystemCompleter { path });
        })
    }
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
            let Ok((pos, res)) = completer.complete(line, pos, ctx) else {
                continue;
            };
            let arr = res
                .into_iter()
                .map(|pair| Pair {
                    replacement: pair.replacement + " ",
                    ..pair
                })
                .collect::<Vec<_>>();

            if !arr.is_empty() {
                return Ok((pos, arr));
            }
        }
        Ok((0, Vec::with_capacity(0)))
    }
}

struct BuiltinCompleter;

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
                replacement: (x[pos..]).to_string(),
            })
            .collect::<Vec<_>>();

        Ok((pos, candidates))
    }
}

struct FilesystemCompleter {
    path: String,
}

impl Completer for FilesystemCompleter {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        let read_dir =
            std::fs::read_dir(&self.path).map_err(rustyline::error::ReadlineError::Io)?;

        let mut result = Vec::new();
        for path in read_dir {
            let Ok(dir_entry) = path else {
                continue;
            };
            let Ok(val) = dir_entry.file_name().into_string() else {
                continue;
            };
            if val.starts_with(line) {
                let pair = Pair {
                    display: val.clone(),
                    replacement: (val[pos..]).to_string(),
                };
                result.push(pair);
            }
        }
        Ok((pos, result))
    }
}
