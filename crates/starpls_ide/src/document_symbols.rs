use starpls_bazel::APIContext;
use starpls_common::parse;
use starpls_common::Db;
use starpls_common::File;
use starpls_common::FileId;
use starpls_hir::ScopeDef;
use starpls_hir::Semantics;
use starpls_syntax::ast::AstNode;
use starpls_syntax::ast::{self};
use starpls_syntax::TextRange;

use crate::Database;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SymbolKind {
    File,
    Module,
    Namespace,
    Package,
    Class,
    Method,
    Property,
    Field,
    Constructor,
    Enum,
    Interface,
    Function,
    Variable,
    Constant,
    String,
    Number,
    Boolean,
    Array,
    Object,
    Key,
    Null,
    EnumMember,
    Struct,
    Event,
    Operator,
    TypeParameter,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SymbolTag {
    Deprecated,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DocumentSymbol {
    pub name: String,
    pub detail: Option<String>,
    pub kind: SymbolKind,
    pub tags: Option<Vec<SymbolTag>>,
    pub range: TextRange,
    pub selection_range: TextRange,
    pub children: Option<Vec<DocumentSymbol>>,
}

pub(crate) fn document_symbols(db: &Database, file_id: FileId) -> Option<Vec<DocumentSymbol>> {
    let sema = Semantics::new(db);
    let file = db.get_file(file_id)?;
    let scope = sema.scope_for_module(file);
    let mut symbols = scope
        .names()
        .filter_map(|(name, def)| {
            // TODO(withered-magic): Filter out defs not from our current file (e.g. defs from the prelude).
            let range = def.syntax_node_ptr(db)?.value.text_range();
            Some(DocumentSymbol {
                name: name.as_str().to_string(),
                detail: None,
                kind: match def {
                    ScopeDef::Callable(_) => SymbolKind::Function,
                    ScopeDef::Variable(_) => SymbolKind::Variable,
                    _ => return None,
                },
                tags: None,
                range,
                selection_range: range,
                children: None,
            })
        })
        .collect();
    if file.api_context(db) == Some(APIContext::Build) {
        add_target_symbols(db, file, &mut symbols);
    }

    symbols.sort_by(|s1, s2| s1.range.start().cmp(&s2.range.start()));
    Some(symbols)
}

fn add_target_symbols(db: &Database, file: File, acc: &mut Vec<DocumentSymbol>) {
    let root = parse(db, file).syntax(db);
    let targets = root.children().filter_map(|child| {
        let expr = ast::CallExpr::cast(child)?;
        let range = expr.syntax().text_range();
        let name = expr
            .arguments()
            .into_iter()
            .flat_map(|args| args.arguments())
            .find_map(|arg| match arg {
                ast::Argument::Keyword(arg) => {
                    if arg.name()?.name()?.text() != "name" {
                        return None;
                    }
                    let lit = match arg.expr()? {
                        ast::Expression::Literal(lit) => lit,
                        _ => return None,
                    };
                    match lit.kind() {
                        ast::LiteralKind::String(s) => s.value(),
                        _ => None,
                    }
                }
                _ => None,
            })?;
        Some(DocumentSymbol {
            name: format!(":{}", name),
            detail: None,
            kind: SymbolKind::Variable,
            tags: None,
            range,
            selection_range: range,
            children: None,
        })
    });
    acc.extend(targets);
}

#[cfg(test)]
mod tests {
    use expect_test::expect;
    use expect_test::Expect;
    use starpls_bazel::APIContext;
    use starpls_common::Dialect;
    use starpls_common::FileInfo;
    use starpls_hir::Fixture;

    use crate::Analysis;

    fn check(input: &str, expect: Expect) {
        let (mut analysis, loader) = Analysis::new_for_test();
        let mut fixture = Fixture::new(&mut analysis.db);
        let file_id = fixture.add_file_with_options(
            &mut analysis.db,
            "BUILD.bazel",
            input,
            Dialect::Bazel,
            Some(FileInfo::Bazel {
                api_context: APIContext::Build,
                is_external: false,
            }),
        );
        loader.add_files_from_fixture(&analysis.db, &fixture);

        let symbols = analysis
            .snapshot()
            .document_symbols(file_id)
            .unwrap()
            .unwrap();
        let mut actual = String::new();
        for symbol in symbols {
            actual.push_str(&format!("{:?}", symbol));
            actual.push('\n');
        }
        expect.assert_eq(&actual);
    }

    #[test]
    fn test_none() {
        check(r#""#, expect![]);
    }

    #[test]
    fn test_variables_and_functions() {
        check(
            r#"s = "abc"

def foo():
    pass
"#,
            expect![[r#"
                DocumentSymbol { name: "s", detail: None, kind: Variable, tags: None, range: 0..1, selection_range: 0..1, children: None }
                DocumentSymbol { name: "foo", detail: None, kind: Function, tags: None, range: 11..31, selection_range: 11..31, children: None }
            "#]],
        );
    }

    #[test]
    fn test_use_last_assignment() {
        check(
            r#"
y = "abc"
x = 123
x = "123"
"#,
            expect![[r#"
                DocumentSymbol { name: "y", detail: None, kind: Variable, tags: None, range: 1..2, selection_range: 1..2, children: None }
                DocumentSymbol { name: "x", detail: None, kind: Variable, tags: None, range: 19..20, selection_range: 19..20, children: None }
            "#]],
        );
    }

    #[test]
    fn test_skip_load_items() {
        check(
            r#"
load("foo.star", "foo")

bar = 1
"#,
            expect![[r#"
                DocumentSymbol { name: "bar", detail: None, kind: Variable, tags: None, range: 26..29, selection_range: 26..29, children: None }
            "#]],
        )
    }

    #[test]
    fn test_targets() {
        check(
            r#"
NUMS = [1, 2, 3]

rust_library(
    name = "starpls_ide",
    srcs = glob(["src/**/*.rs"]),
)

rust_library_test(
    name = "starpls_ide_test",
    crates = ":starpls_ide",
)
"#,
            expect![[r#"
                DocumentSymbol { name: "NUMS", detail: None, kind: Variable, tags: None, range: 1..5, selection_range: 1..5, children: None }
                DocumentSymbol { name: ":starpls_ide", detail: None, kind: Variable, tags: None, range: 19..94, selection_range: 19..94, children: None }
                DocumentSymbol { name: ":starpls_ide_test", detail: None, kind: Variable, tags: None, range: 96..176, selection_range: 96..176, children: None }
            "#]],
        )
    }
}
