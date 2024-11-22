use marker::Marker;

pub use crate::step::Step;
use crate::step::StepEvent;
pub use crate::syntax_kind::SyntaxKind;
use crate::syntax_kind::SyntaxKind::*;
use crate::syntax_kind::SyntaxKindSet;
pub use crate::text::StrStep;
pub use crate::text::StrWithTokens;

mod grammar;
mod marker;
mod step;
mod syntax_kind;
mod text;

#[cfg(test)]
mod tests;

/// Parses a Starlark module from the given sequence of tokens. This function operates on a sequence of
/// non-trivia tokens; use the `.to_input()` method on an instance of `StrWithTokens` to obtain an `Input`
/// to pass to this function.
pub fn parse(input: &Input) -> Output {
    let mut p = Parser::new(input);
    grammar::module(&mut p);
    step::postprocess_step_events(p.events)
}

pub fn parse_type_list(input: &Input) -> Output {
    let mut p = Parser::new(input);
    grammar::type_comment_body(&mut p);
    step::postprocess_step_events(p.events)
}

/// The input to the parser, consisting of a list of tokens.
pub struct Input {
    tokens: Vec<SyntaxKind>,
}

/// The output of the lexer, consisting of a series of steps that can be used to construct the parse tree.
pub struct Output {
    steps: Vec<Step>,
}

/// A parser for Starlark code. It takes a stream of non-trivia tokens as input and processes that stream
/// to construct a parse tree. Because the parser operates only on token types and has no knowledge of
/// text offsets, etc., it instead outputs a series of steps that can be consumed by a separate parse tree
/// builder to construct the final tree.
pub(crate) struct Parser<'a> {
    input: &'a Input,
    events: Vec<StepEvent>,
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(input: &'a Input) -> Self {
        Self {
            input,
            events: Vec::new(),
            pos: 0,
        }
    }

    pub(crate) fn current(&self) -> SyntaxKind {
        self.nth(0)
    }

    pub(crate) fn nth(&self, n: usize) -> SyntaxKind {
        let pos = self.pos + n;
        if pos >= self.input.tokens.len() {
            EOF
        } else {
            self.input.tokens[pos]
        }
    }

    pub(crate) fn at(&self, kind: SyntaxKind) -> bool {
        self.nth_at(0, kind)
    }

    pub(crate) fn nth_at(&self, n: usize, kind: SyntaxKind) -> bool {
        self.nth(n) == kind
    }

    pub(crate) fn eat(&mut self, kind: SyntaxKind) -> bool {
        if !self.at(kind) {
            false
        } else {
            self.push_event(StepEvent::Token { kind });
            self.pos += 1;
            true
        }
    }

    pub(crate) fn bump(&mut self, kind: SyntaxKind) {
        assert!(self.eat(kind));
    }

    pub(crate) fn bump_any(&mut self) {
        if !self.at(EOF) {
            self.push_event(StepEvent::Token {
                kind: self.input.tokens[self.pos],
            });
            self.pos += 1;
        }
    }

    /// Starts a new node in the syntax tree. All nodes and tokens consumed between the call to `start` and the
    /// corresponding invocation of `Marker::complete` belong to the same node.
    pub(crate) fn start(&mut self) -> Marker {
        let pos = self.events.len() as u32;
        self.push_event(StepEvent::Tombstone);
        Marker::new(pos)
    }

    pub(crate) fn push_event(&mut self, event: StepEvent) {
        self.events.push(event);
    }

    pub(crate) fn error<T>(&mut self, message: T)
    where
        T: Into<String>,
    {
        self.events.push(StepEvent::Error {
            message: message.into(),
        });
    }

    pub(crate) fn error_recover_until(
        &mut self,
        message: impl Into<String>,
        recover: SyntaxKindSet,
    ) {
        self.error(message);

        // Start a new ERROR node and consume tokens until we are at either a token specified in the recovery set, or EOF.
        if !self.at(EOF) && !recover.contains(self.current()) {
            let m = self.start();
            while !self.at(EOF) && !recover.contains(self.current()) {
                self.bump_any();
            }
            m.complete(self, ERROR);
        }
    }

    pub(crate) fn at_kinds(&self, set: SyntaxKindSet) -> bool {
        set.contains(self.current())
    }
}
