use marker::Marker;

use crate::{
    step::StepEvent,
    syntax_kind::{SyntaxKind::*, SyntaxKindSet},
};

pub use crate::{step::Step, syntax_kind::SyntaxKind};

mod grammar;
mod marker;
mod step;
mod syntax_kind;
mod text;

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

    pub(crate) fn error_and_bump<T>(&mut self, message: T)
    where
        T: Into<String>,
    {
        self.error(message);

        // Create a new ERROR node to hold the next token.
        let m = self.start();
        self.bump_any();
        m.complete(self, ERROR);
    }

    pub(crate) fn error_recover<T>(&mut self, message: T, recover: SyntaxKindSet)
    where
        T: Into<String>,
    {
        self.error(message);

        // If we aren't at any of the tokens specified in the recovery set, then
        // create a new ERROR node to hold the next token.
        if !recover.contains(self.current()) {
            let m = self.start();
            self.bump_any();
            m.complete(self, ERROR);
        }
    }

    pub(crate) fn expect(&mut self, kind: SyntaxKind) -> bool {
        if !self.eat(kind) {
            self.error(format!("expected {kind:?}"));
            return false;
        }
        true
    }

    pub(crate) fn at_kinds(&self, set: SyntaxKindSet) -> bool {
        set.contains(self.current())
    }
}
