//! Partially replicates the "completions" API in the LSP specification.

use std::collections::HashSet;

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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Edit {
    TextEdit(TextEdit),
    InsertReplaceEdit(InsertReplaceEdit),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TextEdit {
    pub range: TextRange,
    pub new_text: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct InsertReplaceEdit {
    pub new_text: String,
    pub insert: TextRange,
    pub replace: TextRange,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum CompletionMode {
    InsertText(String),
    TextEdit(Edit),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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
    Label {
        file_id: FileId,
        text: Box<str>,
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
    let ctx = CompletionContext::new(db, pos, trigger_character.clone())?;
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

        CompletionAnalysis::String(StringContext::Label { file_id, text }) => {
            if matches!(trigger_character.as_deref(), Some("@")) {
                return None;
            }

            let package = db.resolve_build_file(file_id).unwrap_or_default();
            let is_relative = text.starts_with(':');
            let prefix = strip_last_package_or_target(&text);
            let has_target = text.contains(':');
            let mut seen_packages = HashSet::<&str>::new();

            for target in db.get_all_workspace_targets().iter() {
                let remaining = match if is_relative {
                    target
                        .strip_prefix("//")
                        .and_then(|res| res.strip_prefix(&package))
                        .and_then(|res| res.strip_prefix(prefix))
                } else {
                    target.strip_prefix(prefix)
                } {
                    Some(remaining) => remaining,
                    None => continue,
                };

                if has_target {
                    items.push(CompletionItem {
                        label: remaining.to_string(),
                        kind: CompletionItemKind::Field,
                        mode: None,
                        relevance: CompletionRelevance::VariableOrKeyword,
                        filter_text: None,
                    });
                } else if let Some(index) = remaining.find(['/', ':']) {
                    let package = &remaining[..index];
                    if !package.is_empty() && !seen_packages.contains(package) {
                        seen_packages.insert(package);
                        items.push(CompletionItem {
                            label: package.to_string(),
                            kind: CompletionItemKind::Folder,
                            mode: None,
                            relevance: CompletionRelevance::VariableOrKeyword,
                            filter_text: None,
                        });
                    }
                }
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

        // Check if the current text is potentially a label.
        let text = text.value()?;
        if text.starts_with("//") || text.starts_with(':') {
            return Some(StringContext::Label { file_id, text });
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

fn strip_last_package_or_target(label: &str) -> &str {
    if let Some(index) = label.rfind(&[':', '/']) {
        &label[..index + 1]
    } else {
        label
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Write;

    use expect_test::expect;
    use expect_test::Expect;
    use starpls_bazel::APIContext;
    use starpls_common::Dialect;
    use starpls_common::FileInfo;
    use starpls_test_util::Fixture;

    use crate::completions::CompletionRelevance;
    use crate::AnalysisSnapshot;
    use crate::CompletionItemKind;
    use crate::FilePosition;

    fn check_completions(fixture: &str, expect: Expect) {
        check_completions_with_options(fixture, false, expect);
    }

    fn check_completions_with_options(
        fixture: &str,
        include_builtins_and_keywords: bool,
        expect: Expect,
    ) {
        let fixture = Fixture::parse(fixture);
        let (snap, file_id) = AnalysisSnapshot::from_single_file_with_options(
            &fixture.contents,
            Dialect::Bazel,
            Some(FileInfo::Bazel {
                api_context: APIContext::Bzl,
                is_external: false,
            }),
            [
                "//:foo",
                "//:bar",
                "//bar:bar",
                "//foo:foo",
                "//foo/bar:bar",
                "//foo/bar:baz",
            ]
            .into_iter()
            .map(String::from)
            .collect(),
        );

        let completions = snap
            .completions(
                FilePosition {
                    file_id,
                    pos: fixture.cursor_pos,
                },
                Some("".to_string()),
            )
            .unwrap()
            .unwrap();

        let mut completions = completions
            .into_iter()
            .filter(|item| {
                include_builtins_and_keywords
                    || (item.relevance != CompletionRelevance::Builtin
                        && item.kind != CompletionItemKind::Keyword)
            })
            .collect::<Vec<_>>();
        completions.sort_by(|item1, item2| item1.label.cmp(&item2.label));

        let expected = completions
            .into_iter()
            .fold(String::new(), |mut acc, item| {
                writeln!(acc, "{:?}", item).unwrap();
                acc
            });

        expect.assert_eq(&expected);
    }

    #[test]
    fn test_empty() {
        check_completions_with_options(
            r#"
$0
"#,
            true,
            expect![[r#"
                CompletionItem { label: "False", kind: Keyword, mode: None, filter_text: None, relevance: VariableOrKeyword }
                CompletionItem { label: "None", kind: Keyword, mode: None, filter_text: None, relevance: VariableOrKeyword }
                CompletionItem { label: "True", kind: Keyword, mode: None, filter_text: None, relevance: VariableOrKeyword }
                CompletionItem { label: "abs", kind: Function, mode: None, filter_text: None, relevance: Builtin }
                CompletionItem { label: "all", kind: Function, mode: None, filter_text: None, relevance: Builtin }
                CompletionItem { label: "any", kind: Function, mode: None, filter_text: None, relevance: Builtin }
                CompletionItem { label: "attr", kind: Module, mode: None, filter_text: None, relevance: Builtin }
                CompletionItem { label: "bool", kind: Function, mode: None, filter_text: None, relevance: Builtin }
                CompletionItem { label: "bytes", kind: Function, mode: None, filter_text: None, relevance: Builtin }
                CompletionItem { label: "def", kind: Keyword, mode: None, filter_text: None, relevance: VariableOrKeyword }
                CompletionItem { label: "dict", kind: Function, mode: None, filter_text: None, relevance: Builtin }
                CompletionItem { label: "dir", kind: Function, mode: None, filter_text: None, relevance: Builtin }
                CompletionItem { label: "enumerate", kind: Function, mode: None, filter_text: None, relevance: Builtin }
                CompletionItem { label: "fail", kind: Function, mode: None, filter_text: None, relevance: Builtin }
                CompletionItem { label: "float", kind: Function, mode: None, filter_text: None, relevance: Builtin }
                CompletionItem { label: "for", kind: Keyword, mode: None, filter_text: None, relevance: VariableOrKeyword }
                CompletionItem { label: "getattr", kind: Function, mode: None, filter_text: None, relevance: Builtin }
                CompletionItem { label: "hasattr", kind: Function, mode: None, filter_text: None, relevance: Builtin }
                CompletionItem { label: "hash", kind: Function, mode: None, filter_text: None, relevance: Builtin }
                CompletionItem { label: "if", kind: Keyword, mode: None, filter_text: None, relevance: VariableOrKeyword }
                CompletionItem { label: "int", kind: Function, mode: None, filter_text: None, relevance: Builtin }
                CompletionItem { label: "lambda", kind: Keyword, mode: None, filter_text: None, relevance: VariableOrKeyword }
                CompletionItem { label: "len", kind: Function, mode: None, filter_text: None, relevance: Builtin }
                CompletionItem { label: "licenses", kind: Function, mode: None, filter_text: None, relevance: Builtin }
                CompletionItem { label: "list", kind: Function, mode: None, filter_text: None, relevance: Builtin }
                CompletionItem { label: "load", kind: Keyword, mode: None, filter_text: None, relevance: VariableOrKeyword }
                CompletionItem { label: "max", kind: Function, mode: None, filter_text: None, relevance: Builtin }
                CompletionItem { label: "min", kind: Function, mode: None, filter_text: None, relevance: Builtin }
                CompletionItem { label: "module_extension", kind: Function, mode: None, filter_text: None, relevance: Builtin }
                CompletionItem { label: "package", kind: Function, mode: None, filter_text: None, relevance: Builtin }
                CompletionItem { label: "pass", kind: Keyword, mode: None, filter_text: None, relevance: VariableOrKeyword }
                CompletionItem { label: "print", kind: Function, mode: None, filter_text: None, relevance: Builtin }
                CompletionItem { label: "provider", kind: Function, mode: None, filter_text: None, relevance: Builtin }
                CompletionItem { label: "range", kind: Function, mode: None, filter_text: None, relevance: Builtin }
                CompletionItem { label: "repository_rule", kind: Function, mode: None, filter_text: None, relevance: Builtin }
                CompletionItem { label: "repr", kind: Function, mode: None, filter_text: None, relevance: Builtin }
                CompletionItem { label: "reversed", kind: Function, mode: None, filter_text: None, relevance: Builtin }
                CompletionItem { label: "rule", kind: Function, mode: None, filter_text: None, relevance: Builtin }
                CompletionItem { label: "sorted", kind: Function, mode: None, filter_text: None, relevance: Builtin }
                CompletionItem { label: "str", kind: Function, mode: None, filter_text: None, relevance: Builtin }
                CompletionItem { label: "struct", kind: Function, mode: None, filter_text: None, relevance: Builtin }
                CompletionItem { label: "tag_class", kind: Function, mode: None, filter_text: None, relevance: Builtin }
                CompletionItem { label: "tuple", kind: Function, mode: None, filter_text: None, relevance: Builtin }
                CompletionItem { label: "type", kind: Function, mode: None, filter_text: None, relevance: Builtin }
                CompletionItem { label: "zip", kind: Function, mode: None, filter_text: None, relevance: Builtin }
            "#]],
        );
    }

    #[test]
    fn test_parameters() {
        check_completions(
            r#"
abc = 1
def foo(x, y):
    pass
    x + $0
"#,
            expect![[r#"
                CompletionItem { label: "abc", kind: Variable, mode: None, filter_text: None, relevance: VariableOrKeyword }
                CompletionItem { label: "foo", kind: Function, mode: None, filter_text: None, relevance: VariableOrKeyword }
                CompletionItem { label: "x", kind: Variable, mode: None, filter_text: None, relevance: VariableOrKeyword }
                CompletionItem { label: "y", kind: Variable, mode: None, filter_text: None, relevance: VariableOrKeyword }
            "#]],
        );
    }

    #[test]
    fn test_arguments() {
        check_completions(
            r#"
def foo(x, y):
    pass

foo(
    $0
)
"#,
            expect![[r#"
                CompletionItem { label: "foo", kind: Function, mode: None, filter_text: None, relevance: VariableOrKeyword }
                CompletionItem { label: "x=", kind: Variable, mode: Some(InsertText("x = ")), filter_text: None, relevance: Parameter }
                CompletionItem { label: "y=", kind: Variable, mode: Some(InsertText("y = ")), filter_text: None, relevance: Parameter }
            "#]],
        );
    }

    #[test]
    fn test_variables() {
        check_completions(
            r#"
x = 1
y = 2
$0
"#,
            expect![[r#"
                CompletionItem { label: "x", kind: Variable, mode: None, filter_text: None, relevance: VariableOrKeyword }
                CompletionItem { label: "y", kind: Variable, mode: None, filter_text: None, relevance: VariableOrKeyword }
            "#]],
        );
    }

    #[test]
    fn test_fields() {
        check_completions(
            r#"
foo = struct(x = 1, y = 2)
foo.$0
"#,
            expect![[r#"
                CompletionItem { label: "x", kind: Field, mode: None, filter_text: None, relevance: VariableOrKeyword }
                CompletionItem { label: "y", kind: Field, mode: None, filter_text: None, relevance: VariableOrKeyword }
            "#]],
        );
    }

    #[test]
    fn test_dict_keys() {
        check_completions(
            r#"
d = {"a": 1, "b": 2}
d["$0"]
"#,
            expect![[r#"
                CompletionItem { label: "a", kind: Constant, mode: None, filter_text: None, relevance: VariableOrKeyword }
                CompletionItem { label: "b", kind: Constant, mode: None, filter_text: None, relevance: VariableOrKeyword }
            "#]],
        );
    }

    #[test]
    fn test_label_completions_1() {
        check_completions(
            r#"
label = "//$0"
"#,
            expect![[r#"
                CompletionItem { label: "bar", kind: Folder, mode: None, filter_text: None, relevance: VariableOrKeyword }
                CompletionItem { label: "foo", kind: Folder, mode: None, filter_text: None, relevance: VariableOrKeyword }
            "#]],
        );
    }

    #[test]
    fn test_label_completions_2() {
        check_completions(
            r#"
label = "//fo$0"
"#,
            expect![[r#"
                CompletionItem { label: "bar", kind: Folder, mode: None, filter_text: None, relevance: VariableOrKeyword }
                CompletionItem { label: "foo", kind: Folder, mode: None, filter_text: None, relevance: VariableOrKeyword }
            "#]],
        );
    }

    #[test]
    fn test_label_completions_3() {
        check_completions(
            r#"
label = "//:$0"
"#,
            expect![[r#"
                CompletionItem { label: "bar", kind: Field, mode: None, filter_text: None, relevance: VariableOrKeyword }
                CompletionItem { label: "foo", kind: Field, mode: None, filter_text: None, relevance: VariableOrKeyword }
            "#]],
        );
    }

    #[test]
    fn test_label_completions_4() {
        check_completions(
            r#"
label = "//:f$0"
"#,
            expect![[r#"
                CompletionItem { label: "bar", kind: Field, mode: None, filter_text: None, relevance: VariableOrKeyword }
                CompletionItem { label: "foo", kind: Field, mode: None, filter_text: None, relevance: VariableOrKeyword }
            "#]],
        );
    }

    #[test]
    fn test_label_completions_5() {
        check_completions(
            r#"
label = "//foo:$0"
"#,
            expect![[r#"
                CompletionItem { label: "foo", kind: Field, mode: None, filter_text: None, relevance: VariableOrKeyword }
            "#]],
        );
    }

    #[test]
    fn test_label_completions_6() {
        check_completions(
            r#"
label = "//foo/$0"
"#,
            expect![[r#"
                CompletionItem { label: "bar", kind: Folder, mode: None, filter_text: None, relevance: VariableOrKeyword }
            "#]],
        );
    }

    #[test]
    fn test_label_completions_7() {
        check_completions(
            r#"
label = "//foo/bar:b$0"
"#,
            expect![[r#"
                CompletionItem { label: "bar", kind: Field, mode: None, filter_text: None, relevance: VariableOrKeyword }
                CompletionItem { label: "baz", kind: Field, mode: None, filter_text: None, relevance: VariableOrKeyword }
            "#]],
        );
    }
}
