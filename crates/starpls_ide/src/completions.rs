//! Partially replicates the "completions" API in the LSP specification.

use crate::FilePosition;
use starpls_common::{parse, FileId};
use starpls_hir::{Db, Declaration, Name, Param, Resolver, Semantics, Type};
use starpls_syntax::{
    ast::{self, AstNode, AstToken},
    parse_module,
    SyntaxKind::*,
    SyntaxNode, TextRange, TextSize,
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
    relevance: CompletionRelevance,
}

impl CompletionItem {
    pub fn sort_text(&self) -> String {
        format!("{}-{}", self.relevance as u16, self.label)
    }
}

pub struct TextEdit {
    pub range: TextRange,
    pub new_text: String,
}

pub enum CompletionMode {
    InsertText(String),
    TextEdit(TextEdit),
}

pub enum CompletionItemKind {
    Function,
    Variable,
    Keyword,
    Class,
    File,
    Folder,
}

#[repr(u16)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CompletionRelevance {
    Parameter,
    VariableOrKeyword,
}

enum CompletionAnalysis {
    Name(NameContext),
    NameRef(NameRefContext),
    String(StringContext),
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

enum StringContext {
    LoadModule { file_id: FileId, text: ast::String },
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
                    relevance: CompletionRelevance::Parameter,
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
                        relevance: CompletionRelevance::VariableOrKeyword,
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
                    relevance: CompletionRelevance::VariableOrKeyword,
                })
            }
        }
        CompletionAnalysis::Type => {
            for name in BUILTIN_TYPE_NAMES.iter() {
                items.push(CompletionItem {
                    label: name.to_string(),
                    kind: CompletionItemKind::Class,
                    mode: None,
                    relevance: CompletionRelevance::VariableOrKeyword,
                })
            }
        }
        CompletionAnalysis::String(StringContext::LoadModule { file_id, text }) => {
            let (value, offset) = text.value_and_offset()?;
            let token_start = text.syntax().text_range().start() + TextSize::from(offset);
            for candidate in db.list_load_candidates(&value, *file_id).ok()?? {
                let start =
                    TextSize::from(value.rfind('/').map(|start| start + 1).unwrap_or(0) as u32);
                items.push(CompletionItem {
                    label: candidate.path.clone(),
                    kind: CompletionItemKind::File,
                    mode: Some(CompletionMode::TextEdit(TextEdit {
                        range: TextRange::new(
                            start + token_start,
                            TextSize::from(value.len() as u32) + token_start,
                        ),
                        new_text: candidate.path,
                    })),
                    relevance: CompletionRelevance::VariableOrKeyword,
                });
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
            relevance: CompletionRelevance::VariableOrKeyword,
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
            relevance: CompletionRelevance::VariableOrKeyword,
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

fn maybe_str_context(file_id: FileId, root: &SyntaxNode, pos: TextSize) -> Option<StringContext> {
    let token = root.token_at_offset(pos).right_biased()?;
    let text = ast::String::cast(token.clone())?;
    let parent = token.parent()?;

    if ast::LoadModule::can_cast(parent.kind()) {
        Some(StringContext::LoadModule { file_id, text })
    } else {
        None
    }
}

impl CompletionContext {
    fn new(db: &dyn Db, FilePosition { file_id, pos }: FilePosition) -> Option<Self> {
        // Reparse the file with a dummy identifier inserted at the current offset.
        let sema = Semantics::new(db);
        let file = db.get_file(file_id)?;
        let parse = parse(db, file);

        if let Some(cx) = maybe_str_context(file_id, &parse.syntax(db), pos) {
            return Some(CompletionContext {
                analysis: CompletionAnalysis::String(cx),
            });
        }

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
