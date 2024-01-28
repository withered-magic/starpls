//! Partially replicates the "completions" API in the LSP specification.

use crate::FilePosition;
use starpls_common::parse;
use starpls_hir::{Db, Declaration, Name, Param, Resolver, Semantics, Type};
use starpls_syntax::{
    ast::{self, AstNode},
    parse_module,
    SyntaxKind::*,
};
use std::collections::HashMap;

const COMPLETION_MARKER: &'static str = "__STARPLS_COMPLETION_MARKER";

const BUILTIN_TYPE_NAMES: &[&str] = &[
    "NoneType", "bool", "int", "float", "string", "bytes", "list", "tuple", "dict", "range",
];

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
    Class,
}

enum CompletionAnalysis {
    Name(NameContext),
    NameRef(NameRefContext),
    Type,
}

enum NameContext {
    Def,
    Dot { receiver_ty: Type },
}

struct NameRefContext {
    names: HashMap<Name, Declaration>,
    params: Vec<Param>,
    is_in_def: bool,
    is_in_for: bool,
    is_lone_expr: bool,
    is_loop_variable: bool,
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
            params,
            is_lone_expr,
            is_in_def,
            is_in_for,
            is_loop_variable,
        }) => {
            for name in params.iter().filter_map(|param| param.name(db)) {
                items.push(CompletionItem {
                    label: format!("{}=", name.as_str()),
                    kind: CompletionItemKind::Variable,
                    mode: Some(CompletionMode::InsertText(format!("{} = ", name.as_str()))),
                });
            }
            if !is_loop_variable {
                add_globals(&mut items);
                for (name, decl) in names {
                    items.push(CompletionItem {
                        label: name.to_string(),
                        kind: match decl {
                            Declaration::Function { .. }
                            | Declaration::IntrinsicFunction { .. }
                            | Declaration::BuiltinFunction { .. } => CompletionItemKind::Function,
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
        }
        CompletionAnalysis::Name(NameContext::Dot { receiver_ty }) => {
            for (name, ty) in receiver_ty.fields(db) {
                items.push(CompletionItem {
                    label: name.name(db).to_string(),
                    kind: if ty.is_function() {
                        CompletionItemKind::Function
                    } else {
                        CompletionItemKind::Variable
                    },
                    mode: None,
                })
            }
        }
        CompletionAnalysis::Type => {
            for name in BUILTIN_TYPE_NAMES.iter() {
                items.push(CompletionItem {
                    label: name.to_string(),
                    kind: CompletionItemKind::Class,
                    mode: None,
                })
            }
        }
        _ => {}
    }
    Some(items)
}

pub(crate) fn add_globals(items: &mut Vec<CompletionItem>) {
    let add_global = &mut |global: &'static str| {
        items.push(CompletionItem {
            label: global.to_string(),
            kind: CompletionItemKind::Keyword,
            mode: None,
        })
    };
    add_global("True");
    add_global("False");
    add_global("None");
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
        let sema = Semantics::new(db);
        let file = db.get_file(file_id)?;
        let parse = parse(db, file);
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

        let analysis = if let Some(name_ref) = ast::NameRef::cast(parent.clone()) {
            // TODO(withered-magic): There's probably a better way to traverse up the tree.
            let params = name_ref
                .syntax()
                .parent()
                .and_then(|parent| ast::SimpleArgument::cast(parent))
                .and_then(|arg| arg.syntax().parent())
                .and_then(|parent| ast::Arguments::cast(parent))
                .and_then(|arg| arg.syntax().parent())
                .and_then(|parent| ast::CallExpr::cast(parent))
                .and_then(|expr| expr.callee())
                .and_then(|expr| sema.type_of_expr(file, &expr))
                .map(|ty| ty.params(db))
                .unwrap_or_else(|| vec![]);

            let resolver = Resolver::new_for_offset(db, file, pos);

            let (is_in_def, is_in_for, is_loop_variable) =
                parent.ancestors().map(|node| node.kind()).fold(
                    (false, false, false),
                    |(is_in_def, is_in_for, is_loop_variable), kind| {
                        (
                            is_in_def || kind == DEF_STMT,
                            (is_in_for || (kind == FOR_STMT && !is_in_def)),
                            (is_loop_variable || kind == LOOP_VARIABLES),
                        )
                    },
                );

            let is_lone_expr = parent
                .parent()
                .map(|node| matches!(node.kind(), MODULE | SUITE))
                .unwrap_or(true);
            CompletionAnalysis::NameRef(NameRefContext {
                names: resolver.names(),
                params,
                is_in_def,
                is_in_for,
                is_lone_expr,
                is_loop_variable,
            })
        } else if let Some(name) = ast::Name::cast(parent.clone()) {
            let parent = name.syntax().parent()?;
            CompletionAnalysis::Name(if let Some(expr) = ast::DotExpr::cast(parent) {
                NameContext::Dot {
                    receiver_ty: sema.type_of_expr(file, &expr.expr()?.into())?,
                }
            } else {
                NameContext::Def
            })
        } else if let Some(_) = ast::NamedType::cast(parent) {
            CompletionAnalysis::Type
        } else {
            return None;
        };

        Some(Self { analysis })
    }
}
