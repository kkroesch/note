// src/completer.rs
use rustyline::completion::{Completer, FilenameCompleter, Pair};
use rustyline::{Context, Result};

pub struct CustomCompleter {
    pub file_completer: FilenameCompleter,
}

impl CustomCompleter {
    pub fn new() -> Self {
        Self {
            file_completer: FilenameCompleter::new(),
        }
    }
}

impl Completer for CustomCompleter {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &Context<'_>,
    ) -> Result<(usize, Vec<Self::Candidate>)> {
        if line.starts_with("@source ") {
            self.file_completer.complete(line, pos, ctx)
        } else {
            Ok((0, Vec::new()))
        }
    }
}
