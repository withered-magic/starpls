//! Partially replicates the "completions" API in the LSP specification.

use std::collections::HashMap;

use starpls_common::parse;
use starpls_hir::{lower, Db, Declaration, Name, Resolver};
use starpls_syntax::{
    ast::{self, AstNode},
    parse_module, TextSize,
};

const COMPLETION_MARKER: &'static str = "__STARPLS_COMPLETION_MARKER";

use crate::FilePosition;

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
}

enum CompletionAnalysis {
    Name { names: HashMap<Name, Declaration> },
    Field,
}

struct CompletionContext {
    analysis: CompletionAnalysis,
}

pub(crate) fn completions(db: &dyn Db, pos: FilePosition) -> Option<Vec<CompletionItem>> {
    let ctx = CompletionContext::new(db, pos)?;
    let mut items = Vec::new();

    match &ctx.analysis {
        CompletionAnalysis::Name { names } => {
            for name in names.keys() {
                items.push(CompletionItem {
                    label: name.inner(db).clone(),
                    kind: CompletionItemKind::Variable,
                    mode: None,
                })
            }
        }
        _ => (),
    }
    Some(items)
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

        let analysis = if let Some(_name) = ast::Name::cast(parent.clone()) {
            eprintln!("is name analysis");
            let info = lower(db, parse);
            let resolver = Resolver::new_for_offset(db, info, pos);
            CompletionAnalysis::Name {
                names: resolver.names(),
            }
        } else if let Some(_field) = ast::Field::cast(parent.clone()) {
            CompletionAnalysis::Field
        } else {
            return None;
        };

        Some(Self { analysis })
    }
}
