//! Partially replicates the "completions" API in the LSP specification.

use crate::FilePosition;
use starpls_common::parse;
use starpls_hir::{lower, Db, Declaration, Name, Resolver};
use starpls_syntax::{
    ast::{self, AstNode},
    parse_module,
    SyntaxKind::*,
};
use std::collections::HashMap;

const COMPLETION_MARKER: &'static str = "__STARPLS_COMPLETION_MARKER";

pub struct CompletionItem {
    pub label: String,
    pub kind: CompletionItemKind,
    pub mode: Option<CompletionMode>,
}

pub enum CompletionMode {
    InsertText(String),
}

pub enum CompletionItemKind {
    Function,
    Variable,
    Keyword,
}

enum CompletionAnalysis {
    Name,
    NameRef(NameRefContext),
}

struct NameRefContext {
    names: HashMap<Name, Declaration>,
    is_in_def: bool,
    is_in_for: bool,
    is_lone_expr: bool,
}

struct CompletionContext {
    analysis: CompletionAnalysis,
}

pub(crate) fn completions(db: &dyn Db, pos: FilePosition) -> Option<Vec<CompletionItem>> {
    let ctx = CompletionContext::new(db, pos)?;
    let mut items = Vec::new();

    match &ctx.analysis {
        CompletionAnalysis::NameRef(NameRefContext {
            names,
            is_lone_expr,
            is_in_def,
            is_in_for,
        }) => {
            for (name, decl) in names {
                items.push(CompletionItem {
                    label: name.inner(db).clone(),
                    kind: match decl {
                        Declaration::Function { .. } => CompletionItemKind::Function,
                        Declaration::Variable { .. } | Declaration::Parameter { .. } => {
                            CompletionItemKind::Variable
                        }
                        _ => CompletionItemKind::Variable,
                    },
                    mode: None,
                });
            }
            if *is_lone_expr {
                add_keywords(&mut items, *is_in_def, *is_in_for);
            }
        }
        _ => (),
    }
    Some(items)
}

fn add_keywords(items: &mut Vec<CompletionItem>, is_in_def: bool, is_in_for: bool) {
    let add_keyword = &mut |keyword: &'static str| {
        items.push(CompletionItem {
            label: keyword.to_string(),
            kind: CompletionItemKind::Keyword,
            mode: None,
        })
    };
    add_keyword("def");
    add_keyword("if");
    add_keyword("for");
    add_keyword("load");
    add_keyword("pass");

    if is_in_def {
        add_keyword("return");
    }

    if is_in_for {
        add_keyword("break");
        add_keyword("continue");
    }
}

impl CompletionContext {
    fn new(db: &dyn Db, FilePosition { file_id, pos }: FilePosition) -> Option<Self> {
        // Reparse the file with a dummy identifier inserted at the current offset.
        let parse = parse(db, db.get_file(file_id)?);
        let mut text = parse.syntax(db).text().to_string();
        let insert_pos: usize = pos.into();
        if insert_pos > text.len() {
            return None;
        }
        text.insert_str(insert_pos, COMPLETION_MARKER);
        let modified_parse = parse_module(&text, &mut |_| {});

        // Find the node in the modified parse tree corresponding to the original node.
        let parent = modified_parse
            .syntax()
            .token_at_offset(pos)
            .right_biased()?
            .parent()?;

        let analysis = if let Some(_name_ref) = ast::NameRef::cast(parent.clone()) {
            let info = lower(db, parse);
            let resolver = Resolver::new_for_offset(db, info, pos);
            let (is_in_def, is_in_for) = parent.ancestors().map(|node| node.kind()).fold(
                (false, false),
                |(is_in_def, is_in_for), kind| {
                    (
                        is_in_def || kind == DEF_STMT,
                        (is_in_for || (kind == FOR_STMT && !is_in_def)),
                    )
                },
            );
            let is_lone_expr = parent
                .parent()
                .map(|node| matches!(node.kind(), MODULE | SUITE))
                .unwrap_or(true);
            CompletionAnalysis::NameRef(NameRefContext {
                names: resolver.names(),
                is_in_def,
                is_in_for,
                is_lone_expr,
            })
        } else if let Some(_name) = ast::Name::cast(parent.clone()) {
            CompletionAnalysis::Name
        } else {
            return None;
        };

        Some(Self { analysis })
    }
}
