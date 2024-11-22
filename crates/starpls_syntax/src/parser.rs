use std::marker::PhantomData;

use rowan::ast::AstNode;
use rowan::GreenNode;
use rowan::GreenNodeBuilder;
use rowan::Language;
use rowan::TextRange;
use rowan::TextSize;
use starpls_parser::parse;
use starpls_parser::parse_type_list;
use starpls_parser::StrStep;
use starpls_parser::StrWithTokens;
use starpls_parser::SyntaxKind::*;

use crate::LineIndex;
use crate::Module;
use crate::StarlarkLanguage;
use crate::SyntaxNode;

const TYPE_COMMENT_PREFIX_STR: &str = "# type: ";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyntaxError {
    pub message: String,
    pub range: TextRange,
}

/// The result of parsing a Starlark module and constructing a Rowan syntax tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseTree<T> {
    green: GreenNode,
    _ty: PhantomData<fn() -> T>,
}

impl<T> ParseTree<T> {
    fn new(green: GreenNode) -> Self {
        ParseTree {
            green,
            _ty: PhantomData,
        }
    }

    pub fn syntax(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.green.clone())
    }
}

impl<T: AstNode<Language = StarlarkLanguage>> ParseTree<T> {
    pub fn tree(&self) -> T {
        T::cast(self.syntax()).unwrap()
    }
}

pub fn parse_module(input: &str, errors_sink: &mut dyn FnMut(SyntaxError)) -> ParseTree<Module> {
    let str_with_tokens = StrWithTokens::new(input);
    let output = parse(&str_with_tokens.to_input());
    let mut builder = GreenNodeBuilder::new();

    add_lexer_errors(&str_with_tokens, errors_sink);

    str_with_tokens.build_with_trivia(output, &mut |str_step| match str_step {
        StrStep::Start { kind } => {
            builder.start_node(StarlarkLanguage::kind_to_raw(kind));
        }
        StrStep::Finish => {
            builder.finish_node();
        }
        StrStep::Token { kind, text, pos } => {
            if kind == COMMENT && text.starts_with(TYPE_COMMENT_PREFIX_STR) {
                build_type_comment(
                    &mut builder,
                    text,
                    str_with_tokens.token_pos(pos) as usize,
                    errors_sink,
                );
                return;
            }
            builder.token(StarlarkLanguage::kind_to_raw(kind), text);
        }
        StrStep::Error { message, pos } => {
            let token_pos = str_with_tokens.token_pos(pos);
            errors_sink(SyntaxError {
                message,
                range: TextRange::new(TextSize::new(token_pos), TextSize::new(token_pos)),
            });
        }
    });

    let green_node = builder.finish();

    // The root of the parse tree must always be a `Module`.
    assert_eq!(green_node.kind(), StarlarkLanguage::kind_to_raw(MODULE));

    ParseTree::new(green_node)
}

fn build_type_comment(
    builder: &mut GreenNodeBuilder,
    text: &str,
    text_start: usize,
    errors_sink: &mut dyn FnMut(SyntaxError),
) {
    builder.start_node(StarlarkLanguage::kind_to_raw(TYPE_COMMENT));
    builder.token(
        StarlarkLanguage::kind_to_raw(TYPE_COMMENT_PREFIX),
        TYPE_COMMENT_PREFIX_STR,
    );

    let str_with_tokens =
        StrWithTokens::new_for_type_comment(&text[TYPE_COMMENT_PREFIX_STR.len()..]);
    let output = parse_type_list(&str_with_tokens.to_input());
    add_lexer_errors(&str_with_tokens, errors_sink);

    str_with_tokens.build_with_trivia(output, &mut |str_step| match str_step {
        StrStep::Start { kind } => builder.start_node(StarlarkLanguage::kind_to_raw(kind)),
        StrStep::Finish => builder.finish_node(),
        StrStep::Token { kind, text, .. } => {
            builder.token(StarlarkLanguage::kind_to_raw(kind), text)
        }
        StrStep::Error { message, pos } => {
            let offset = ((text_start + TYPE_COMMENT_PREFIX_STR.len()) as u32
                + str_with_tokens.token_pos(pos))
            .into();
            errors_sink(SyntaxError {
                message,
                range: TextRange::new(offset, offset),
            })
        }
    });

    builder.finish_node();
}

fn add_lexer_errors(str_with_tokens: &StrWithTokens, errors_sink: &mut dyn FnMut(SyntaxError)) {
    for lexer_error in str_with_tokens.lexer_errors() {
        errors_sink(SyntaxError {
            message: lexer_error.message.to_string(),
            range: TextRange::new(
                TextSize::new(lexer_error.start as u32),
                TextSize::new(lexer_error.end as u32),
            ),
        });
    }
}

pub fn line_index(input: &str) -> LineIndex {
    LineIndex::new(input)
}
