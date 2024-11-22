//! Partially replicates the "completions" API in the LSP specification.

use rustc_hash::FxHashMap;
use starpls_common::parse;
use starpls_common::FileId;
use starpls_common::LoadItemCandidateKind;
use starpls_hir::Db;
use starpls_hir::Name;
use starpls_hir::Param;
use starpls_hir::ScopeDef;
use starpls_hir::Semantics;
use starpls_hir::Type;
use starpls_syntax::ast::AstNode;
use starpls_syntax::ast::AstToken;
use starpls_syntax::ast::{self};
use starpls_syntax::parse_module;
use starpls_syntax::SyntaxKind::*;
use starpls_syntax::SyntaxNode;
use starpls_syntax::TextRange;
use starpls_syntax::TextSize;

use crate::FilePosition;

const COMPLETION_MARKER: &str = "__STARPLS_COMPLETION_MARKER";

const BUILTIN_TYPE_NAMES: &[&str] = &[
    "NoneType", "bool", "int", "float", "string", "bytes", "list", "tuple", "dict", "range",
];

#[derive(Debug)]
pub struct CompletionItem {
    pub label: String,
    pub kind: CompletionItemKind,
    pub mode: Option<CompletionMode>,
    pub filter_text: Option<String>,
    relevance: CompletionRelevance,
}

impl CompletionItem {
    pub fn sort_text(&self) -> String {
        format!("{}-{}", self.relevance as u16, self.label)
    }
}

#[derive(Debug)]
pub enum Edit {
    TextEdit(TextEdit),
    InsertReplaceEdit(InsertReplaceEdit),
}

#[derive(Debug)]
pub struct TextEdit {
    pub range: TextRange,
    pub new_text: String,
}

#[derive(Debug)]
pub struct InsertReplaceEdit {
    pub new_text: String,
    pub insert: TextRange,
    pub replace: TextRange,
}

#[derive(Debug)]
pub enum CompletionMode {
    InsertText(String),
    TextEdit(Edit),
}

#[derive(Debug)]
pub enum CompletionItemKind {
    Function,
    Field,
    Variable,
    Class,
    Module,
    Keyword,
    File,
    Folder,
    Constant,
}

#[repr(u16)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CompletionRelevance {
    Parameter,
    VariableOrKeyword,
    Builtin,
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
    names: FxHashMap<Name, ScopeDef>,
    params: Vec<Param>,
    is_in_def: bool,
    is_in_for: bool,
    is_lone_expr: bool,
    is_loop_variable: bool,
}

enum StringContext {
    LoadModule {
        file_id: FileId,
        text: ast::String,
    },
    LoadItem {
        file_id: FileId,
        load_stmt: ast::LoadStmt,
    },
    DictKey {
        file_id: FileId,
        lhs: ast::Expression,
    },
}

struct CompletionContext {
    analysis: CompletionAnalysis,
}

pub(crate) fn completions(
    db: &dyn Db,
    pos: FilePosition,
    trigger_character: Option<String>,
) -> Option<Vec<CompletionItem>> {
    let ctx = CompletionContext::new(db, pos, trigger_character)?;
    let mut items = Vec::new();

    match ctx.analysis {
        CompletionAnalysis::NameRef(NameRefContext {
            names,
            params,
            is_lone_expr,
            is_in_def,
            is_in_for,
            is_loop_variable,
        }) => {
            // Add completions for parameter names (excluding arg list and kwarg dict parameters).
            for name in params
                .iter()
                .filter(|param| {
                    !param.is_args_list(db)
                        && !param.is_kwargs_dict(db)
                        && !param.is_positional_only(db)
                })
                .filter_map(|param| match param.name(db) {
                    Some(name) if !name.is_missing() => Some(name),
                    _ => None,
                })
            {
                items.push(CompletionItem {
                    label: format!("{}=", name.as_str()),
                    kind: CompletionItemKind::Variable,
                    mode: Some(CompletionMode::InsertText(format!("{} = ", name.as_str()))),
                    relevance: CompletionRelevance::Parameter,
                    filter_text: None,
                });
            }

            if !is_loop_variable {
                add_globals(&mut items);
                for (name, decl) in names {
                    items.push(CompletionItem {
                        label: name.to_string(),
                        kind: match &decl {
                            ScopeDef::Callable(_) => CompletionItemKind::Function,
                            def if def.ty(db).is_callable() => CompletionItemKind::Function,
                            // All the global values in the Bazel builtins are modules.
                            ScopeDef::Variable(it) if !it.is_user_defined() => {
                                CompletionItemKind::Module
                            }
                            _ => CompletionItemKind::Variable,
                        },
                        mode: None,
                        relevance: if decl.is_user_defined() {
                            CompletionRelevance::VariableOrKeyword
                        } else {
                            CompletionRelevance::Builtin
                        },
                        filter_text: None,
                    });
                }

                if is_lone_expr {
                    add_keywords(&mut items, is_in_def, is_in_for);
                }
            }
        }
        CompletionAnalysis::Name(NameContext::Dot { receiver_ty }) => {
            for (name, ty) in receiver_ty.fields(db) {
                items.push(CompletionItem {
                    label: name.name(db).to_string(),
                    kind: if ty.is_callable() {
                        CompletionItemKind::Function
                    } else {
                        CompletionItemKind::Field
                    },
                    mode: None,
                    relevance: CompletionRelevance::VariableOrKeyword,
                    filter_text: None,
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
                    filter_text: None,
                })
            }
        }
        CompletionAnalysis::String(StringContext::LoadModule { file_id, text }) => {
            let (value, offset) = text.value_and_offset()?;
            let token_start = text.syntax().text_range().start() + TextSize::from(offset);
            for candidate in db.list_load_candidates(&value, file_id).ok()?? {
                let start = TextSize::from(
                    value
                        .rfind(&['/', ':', '@'])
                        .map(|start| {
                            if candidate.replace_trailing_slash {
                                start
                            } else {
                                start + 1
                            }
                        })
                        .unwrap_or(0) as u32,
                );
                let end = TextSize::from(value.len() as u32);
                let (edit, filter_text) = if candidate.replace_trailing_slash {
                    (
                        Edit::InsertReplaceEdit(InsertReplaceEdit {
                            new_text: candidate.path.clone(),
                            insert: TextRange::new(token_start + start, token_start + end),
                            replace: TextRange::new(token_start + start, token_start + end),
                        }),
                        Some("/".to_string()),
                    )
                } else {
                    (
                        Edit::TextEdit(TextEdit {
                            range: TextRange::new(token_start + start, token_start + end),
                            new_text: candidate.path.clone(),
                        }),
                        None,
                    )
                };

                items.push(CompletionItem {
                    label: candidate.path,
                    kind: match candidate.kind {
                        LoadItemCandidateKind::Directory => CompletionItemKind::Folder,
                        LoadItemCandidateKind::File => CompletionItemKind::File,
                    },
                    mode: Some(CompletionMode::TextEdit(edit)),
                    relevance: CompletionRelevance::VariableOrKeyword,
                    filter_text,
                });
            }
        }
        CompletionAnalysis::String(StringContext::LoadItem { file_id, load_stmt }) => {
            let sema = Semantics::new(db);
            let file = db.get_file(file_id)?;
            let loaded_file = sema.resolve_load_stmt(file, &load_stmt)?;
            let scope = sema.scope_for_module(loaded_file);
            for (name, def) in scope.exports() {
                items.push(CompletionItem {
                    label: name.to_string(),
                    kind: match &def {
                        ScopeDef::Callable(it) if it.is_user_defined() => {
                            CompletionItemKind::Function
                        }
                        ScopeDef::Variable(it) if it.is_user_defined() => {
                            if def.ty(db).is_callable() {
                                CompletionItemKind::Function
                            } else {
                                CompletionItemKind::Variable
                            }
                        }
                        _ => continue,
                    },
                    mode: None,
                    relevance: CompletionRelevance::VariableOrKeyword,
                    filter_text: None,
                });
            }
        }
        CompletionAnalysis::String(StringContext::DictKey { file_id, lhs }) => {
            let sema = Semantics::new(db);
            let file = db.get_file(file_id)?;
            let ty = sema.type_of_expr(file, &lhs)?;

            for key in ty.known_keys(db)?.into_iter() {
                items.push(CompletionItem {
                    label: key,
                    kind: CompletionItemKind::Constant,
                    mode: None,
                    relevance: CompletionRelevance::VariableOrKeyword,
                    filter_text: None,
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
            filter_text: None,
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
            filter_text: None,
        })
    };
    add_keyword("def");
    add_keyword("if");
    add_keyword("for");
    add_keyword("load");
    add_keyword("pass");
    add_keyword("lambda");

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
        return Some(StringContext::LoadModule { file_id, text });
    } else if ast::LoadItem::can_cast(parent.kind()) {
        let load_stmt = ast::LoadStmt::cast(parent.parent()?)?;
        return Some(StringContext::LoadItem { file_id, load_stmt });
    } else if let Some(expr) = ast::LiteralExpr::cast(parent) {
        if let Some(index_expr) = ast::IndexExpr::cast(expr.syntax().parent()?) {
            if index_expr.index() == Some(ast::Expression::Literal(expr)) {
                return Some(StringContext::DictKey {
                    file_id,
                    lhs: index_expr.lhs()?,
                });
            }
        }
    }

    None
}

impl CompletionContext {
    fn new(
        db: &dyn Db,
        FilePosition { file_id, pos }: FilePosition,
        trigger_character: Option<String>,
    ) -> Option<Self> {
        // Reparse the file with a dummy identifier inserted at the current offset.
        let sema = Semantics::new(db);
        let file = db.get_file(file_id)?;
        let parse = parse(db, file);

        if let Some(cx) = maybe_str_context(file_id, &parse.syntax(db), pos) {
            return Some(CompletionContext {
                analysis: CompletionAnalysis::String(cx),
            });
        }

        if matches!(trigger_character.as_deref(), Some("/" | ":" | "@")) {
            return None;
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
            let args = name_ref
                .syntax()
                .parent()
                .and_then(ast::SimpleArgument::cast)
                .and_then(|arg| arg.syntax().parent())
                .and_then(ast::Arguments::cast);

            let keyword_args = args
                .as_ref()
                .map(|args| {
                    args.arguments()
                        .filter_map(|arg| match arg {
                            ast::Argument::Keyword(kwarg) => kwarg
                                .name()
                                .and_then(|name| name.name())
                                .map(|name| name.text().to_string()),
                            _ => None,
                        })
                        .collect::<Vec<_>>()
                })
                .unwrap_or_else(Vec::new);

            let params = args
                .and_then(|arg| arg.syntax().parent())
                .and_then(ast::CallExpr::cast)
                .and_then(|expr| expr.callee())
                .and_then(|expr| sema.type_of_expr(file, &expr))
                .map(|ty| {
                    ty.params(db)
                        .into_iter()
                        .filter_map(|(param, _)| match param.name(db) {
                            Some(name)
                                if keyword_args.iter().all(|kwarg| kwarg != name.as_str()) =>
                            {
                                Some(param)
                            }
                            _ => None,
                        })
                        .collect()
                })
                .unwrap_or_else(std::vec::Vec::new);

            let scope = sema.scope_for_offset(file, pos);

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
                names: scope.names().collect(),
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
                    receiver_ty: sema.type_of_expr(file, &expr.expr()?)?,
                }
            } else {
                NameContext::Def
            })
        } else if ast::PathType::cast(parent).is_some() {
            CompletionAnalysis::Type
        } else {
            return None;
        };

        Some(Self { analysis })
    }
}
