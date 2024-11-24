use std::fmt::Write;

use starpls_common::parse;
use starpls_common::Db as _;
use starpls_hir::DisplayWithDb;
use starpls_hir::Semantics;
use starpls_hir::Type;
use starpls_syntax::ast::AstNode;
use starpls_syntax::ast::{self};
use starpls_syntax::SyntaxKind::*;
use starpls_syntax::TextRange;
use starpls_syntax::T;

use crate::util::pick_best_token;
use crate::util::unindent_doc;
use crate::Database;
use crate::FilePosition;

mod docs;

pub struct Markup {
    pub value: String,
}

pub struct Hover {
    pub contents: Markup,
    pub range: Option<TextRange>,
}

impl From<String> for Hover {
    fn from(value: String) -> Self {
        Self {
            contents: Markup { value },
            range: None,
        }
    }
}

pub(crate) fn hover(db: &Database, FilePosition { file_id, pos }: FilePosition) -> Option<Hover> {
    let file = db.get_file(file_id)?;
    let parsed = parse(db, file);
    let sema = Semantics::new(db);
    let token = pick_best_token(parsed.syntax(db).token_at_offset(pos), |kind| match kind {
        T![ident] => 2,
        T!['('] | T![')'] | T!['['] | T![']'] | T!['{'] | T!['}'] => 0,
        kind if kind.is_trivia_token() => 0,
        _ => 1,
    })?;

    // Check for keyword hovers first.
    if token.kind().is_keyword() {
        let text = match token.kind() {
            BREAK => docs::BREAK_DOCS,
            CONTINUE => docs::CONTINUE_DOCS,
            DEF => docs::DEF_DOCS,
            FOR => docs::FOR_DOCS,
            IF => docs::IF_DOCS,
            LOAD => docs::LOAD_DOCS,
            PASS => docs::PASS_DOCS,
            RETURN => docs::RETURN_DOCS,
            _ => return None,
        };
        return Some(text.to_string().into());
    }

    // Otherwise, provide hover information for identifiers.
    let parent = token.parent()?;
    if let Some(expr) = ast::NameRef::cast(parent.clone()) {
        let ty = sema.type_of_expr(file, &expr.clone().into())?;
        return Some(format_for_name(db, expr.name()?.text(), &ty).into());
    } else if let Some(name) = ast::Name::cast(parent.clone()) {
        let parent = name.syntax().parent()?;
        let name_token = name.name()?;
        let name_text = name_token.text();
        if let Some(expr) = ast::DotExpr::cast(parent.clone()) {
            let ty = sema.type_of_expr(file, &expr.expr()?)?;
            let fields = ty.fields(db);
            let (field, field_ty) = fields.into_iter().find_map(|(field, ty)| {
                if field.name(db).as_str() == name_text {
                    Some((field, ty))
                } else {
                    None
                }
            })?;

            // Handle special `def` formatting for methods.
            let mut text = String::from("```python\n");
            if field_ty.is_function() {
                text.push_str("(method) ");
            } else {
                text.push_str("(field) ");
                text.push_str(name_text);
                text.push_str(": ");
            }
            write!(&mut text, "{}", field_ty.display(db)).unwrap();
            text.push_str("\n```\n");

            let doc = field.doc(db);
            if !doc.is_empty() {
                text.push_str(&unindent_doc(&doc));
                text.push('\n');
            }

            return Some(text.into());
        } else if let Some(stmt) = ast::DefStmt::cast(parent.clone()) {
            let func = sema.callable_for_def(file, stmt)?;
            let mut text = String::from("```python\n(function) ");
            write!(text, "{}\n```\n", func.ty(db).display(db)).ok()?;
            if let Some(doc) = func.doc(db) {
                text.push_str(&unindent_doc(&doc));
                text.push('\n');
            }
            return Some(text.into());
        } else if let Some(param) = ast::Parameter::cast(parent.clone()) {
            let (param, ty) = sema.resolve_param(file, &param)?;
            let mut text = String::from("```python\n(parameter) ");
            write!(
                text,
                "{}: {}\n```\n",
                param
                    .name(db)
                    .as_ref()
                    .map(|name| name.as_str())
                    .unwrap_or(""),
                ty.display(db)
            )
            .ok()?;
            if let Some(doc) = param.doc(db) {
                text.push_str(&unindent_doc(&doc));
                text.push('\n');
            }
            return Some(text.into());
        } else if let Some(arg) = ast::Argument::cast(parent) {
            let call = arg
                .syntax()
                .parent()
                .and_then(ast::Arguments::cast)
                .and_then(|args| args.syntax().parent())
                .and_then(ast::CallExpr::cast)?;
            let func = sema.resolve_call_expr(file, &call)?;
            let (name, param, ty) = func.params(db).into_iter().find_map(|(param, ty)| {
                let name = param.name(db)?;
                if name.as_str() == name_text {
                    Some((name, param, ty))
                } else {
                    None
                }
            })?;

            let mut text = format!(
                "```python\n(parameter) {}: {}\n```\n",
                name.as_str(),
                ty.display(db),
            );

            if let Some(doc) = param.doc(db) {
                if !doc.is_empty() {
                    text.push_str(&unindent_doc(&doc));
                    text.push('\n');
                }
            }
            return Some(text.into());
        }
    } else if let Some(segment) = ast::PathSegment::cast(parent.clone()) {
        let path_ty = ast::PathType::cast(segment.syntax().parent()?)?;
        let ty = sema.resolve_path_type(file, &path_ty)?;
        let mut text = format!("```python\n(type) {}\n```\n", ty.display(db));
        if let Some(doc) = ty.doc(db) {
            text.push_str(&unindent_doc(&doc));
            text.push('\n');
        }
        return Some(text.into());
    } else if let Some(load_item) = ast::LoadItem::cast(parent.clone()) {
        let load_item = sema.resolve_load_item(file, &load_item)?;
        let def = sema.def_for_load_item(&load_item)?;
        return Some(format_for_name(db, load_item.name(db).as_str(), &def.value.ty(db)).into());
    } else if let Some(load_module) = ast::LoadModule::cast(parent) {
        let load_stmt = ast::LoadStmt::cast(load_module.syntax().parent()?)?;
        let loaded_file = sema.resolve_load_stmt(file, &load_stmt)?;
        let parsed = parse(db, loaded_file);
        let mut text = format!("```python\n(module) {}\n```\n", token.text());
        if let Some(doc) = parsed.tree(db).doc().and_then(|doc| doc.value()) {
            text.push_str(&unindent_doc(&doc));
            text.push('\n');
        }
        return Some(text.into());
    }

    None
}

fn format_for_name(db: &Database, name: &str, ty: &Type) -> String {
    let mut text = String::from("```python\n");

    // Handle special `def` formatting for function types.
    if ty.is_function() {
        text.push_str("(function) ");
    } else {
        text.push_str("(variable) ");
        text.push_str(name);
        text.push_str(": ");
    }

    write!(&mut text, "{}", ty.display(db)).unwrap();
    text.push_str("\n```\n");

    if let Some(doc) = ty.doc(db) {
        text.push_str(&unindent_doc(&doc));
        text.push('\n');
    }

    text
}

#[cfg(test)]
mod tests {
    use expect_test::expect;
    use expect_test::Expect;
    use starpls_bazel::APIContext;
    use starpls_common::Dialect;
    use starpls_common::FileInfo;
    use starpls_test_util::Fixture;

    use crate::AnalysisSnapshot;
    use crate::FilePosition;

    fn check_hover(fixture: &str, expect: Expect) {
        let fixture = Fixture::parse(fixture);
        let (snap, file_id) = AnalysisSnapshot::from_single_file(
            &fixture.contents,
            Dialect::Bazel,
            Some(FileInfo::Bazel {
                api_context: APIContext::Bzl,
                is_external: false,
            }),
        );

        let hover = snap
            .hover(FilePosition {
                file_id,
                pos: fixture.cursor_pos,
            })
            .unwrap()
            .unwrap();

        expect.assert_eq(&hover.contents.value);
    }

    #[test]
    fn check_variable() {
        check_hover(
            r#"
a$0bc = 123
"#,
            expect![[r#"
                ```python
                (variable) abc: Literal[123]
                ```
            "#]],
        );
    }

    #[test]
    fn check_def_stmt() {
        check_hover(
            r#"
def f$0oo(x, y):
    """Doc string"""
    pass
"#,
            expect![[r#"
                ```python
                (function) def foo(x, y) -> Unknown
                ```
                Doc string  
            "#]],
        );
    }

    #[test]
    fn check_call_expr() {
        check_hover(
            r#"
def foo(x, y):
    """Doc string"""
    pass

f$0oo(1, 2)
"#,
            expect![[r#"
                ```python
                (function) def foo(x, y) -> Unknown
                ```
                Doc string  
            "#]],
        );
    }

    #[test]
    fn check_type() {
        check_hover(
            r#"
x = 1 # type: i$0nt
"#,
            expect![[r#"
                ```python
                (type) int
                ```
            "#]],
        );
    }

    #[test]
    fn check_param() {
        check_hover(
            r#"
def foo(a$0bc):
    """
    Args:
        abc: Easy as 123!
    """
    pass
"#,
            expect![[r#"
                ```python
                (parameter) abc: Unknown
                ```
                Easy as 123!  
            "#]],
        );
    }

    #[test]
    fn check_arg() {
        check_hover(
            r#"
def foo(abc):
    """
    Args:
        abc: Easy as 123!
    """
    pass

foo(a$0bc = 123)
"#,
            expect![[r#"
                ```python
                (parameter) abc: Unknown
                ```
                Easy as 123!  
            "#]],
        );
    }

    #[test]
    fn check_field() {
        check_hover(
            r#"
foo = struct(bar = 123)
foo.b$0ar
"#,
            expect![[r#"
                ```python
                (field) bar: Literal[123]
                ```
            "#]],
        );
    }

    #[test]
    fn check_method() {
        check_hover(
            r#"
def bar():
    pass

foo = struct(bar = bar)
foo.b$0ar
"#,
            expect![[r#"
                ```python
                (method) def bar() -> Unknown
                ```
            "#]],
        );
    }

    #[test]
    fn check_provider_doc() {
        check_hover(
            r#"
Foo$0Info = provider(doc = "The foo provider")
"#,
            expect![[r#"
                ```python
                (variable) FooInfo: Provider[FooInfo]
                ```
                The foo provider  
            "#]],
        );
    }

    #[test]
    fn check_provider_field_doc() {
        check_hover(
            r#"
FooInfo = provider(
    doc = "The foo provider",
    fields = {
        "bar": "The bar field",
    },
)

foo = FooInfo(bar = "bar")
foo.b$0ar
"#,
            expect![[r#"
                ```python
                (field) bar: Unknown
                ```
                The bar field  
            "#]],
        );
    }

    #[test]
    fn check_rule_attr() {
        check_hover(
            r#"
foo = rule(
    attrs = {
        "bar": attr.string(doc = "The bar attr"),
    },
)

foo(
    name = "foo",
    b$0ar = "bar",
)
"#,
            expect![[r#"
                ```python
                (parameter) bar: string
                ```
                The bar attr  
            "#]],
        );
    }
}
