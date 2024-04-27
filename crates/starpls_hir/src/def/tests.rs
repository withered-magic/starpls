use std::collections::HashSet;

use starpls_bazel::{
    env::{make_build_builtins, make_bzl_builtins},
    APIContext,
};
use starpls_common::{Db as _, Dialect, FileId, FileInfo};
use starpls_test_util::parse_fixture;

use crate::{
    def::resolver::Resolver, test_database::TestDatabase, typeck::intrinsics::intrinsic_functions,
    Db as _,
};

fn check_scope(fixture: &str, expected: &[&str]) {
    check_scope_full(fixture, expected, None)
}

fn check_scope_full(fixture: &str, expected: &[&str], prelude: Option<&str>) {
    let mut test_db: TestDatabase = Default::default();
    let file_id = FileId(0);
    let (text, offset, _) = parse_fixture(fixture);
    let file = test_db.create_file(
        file_id,
        Dialect::Bazel,
        Some(FileInfo::Bazel {
            api_context: APIContext::Build,
            is_external: false,
        }),
        text,
    );

    if let Some(prelude) = prelude {
        let prelude_file_id = FileId(1);
        test_db.create_file(
            prelude_file_id,
            Dialect::Bazel,
            Some(FileInfo::Bazel {
                api_context: APIContext::Prelude,
                is_external: false,
            }),
            prelude.to_string(),
        );
        test_db.set_bazel_prelude_file(prelude_file_id);
    }

    // Filter out intrinsic function names as well as the hardcoded `BUILD.bazel` and `.bzl`
    // builtins, which are always added when `APIContext::Build` is the current API context.
    let names_to_filter = intrinsic_functions(&test_db)
        .functions(&test_db)
        .keys()
        .map(|name| name.to_string())
        .chain(
            make_bzl_builtins()
                .global
                .into_iter()
                .map(|global| global.name),
        )
        .chain(
            make_build_builtins()
                .global
                .into_iter()
                .map(|global| global.name),
        )
        .collect::<HashSet<_>>();

    let resolver = Resolver::new_for_offset(&test_db, file, offset);
    let names = resolver.names();
    let mut actual = names
        .keys()
        .filter(|name| !names_to_filter.contains(name.as_str()))
        .map(|name| name.as_str())
        .collect::<Vec<_>>();
    actual.sort();
    assert_eq!(expected, &actual[..]);
}

#[test]
fn smoke_test() {
    check_scope(
        r"
g = 0
def foo():
    x = 1
    y = 2
    $0

def bar():
    pass
",
        &["bar", "foo", "g", "x", "y"],
    )
}

#[test]
fn test_empty_scope() {
    check_scope(
        r"
        $0
    ",
        &[],
    )
}

#[test]
fn test_assign() {
    check_scope(
        r"
a = 0
b, c = 1, 2
d, e = 3, 4
[f, g] = 5, 6
$0
        ",
        &["a", "b", "c", "d", "e", "f", "g"],
    )
}

#[test]
fn test_params() {
    check_scope(
        r"
def foo(x, *args, **kwargs):
    print(x)
    $0
",
        &["args", "foo", "kwargs", "x"],
    )
}

#[test]
fn test_loop_variables() {
    check_scope(
        r"
for x, y in 1, 2, 3:
    print(x, y)
    $0
    ",
        &["x", "y"],
    )
}

#[test]
fn test_lambda() {
    check_scope(
        r"
a = 1
f = lambda x: x + 1$0
    ",
        &["a", "x"],
    )
}

#[test]
fn test_def() {
    check_scope(
        r"def foo():
    x = 1
$0",
        &["foo", "x"],
    )
}

#[test]
fn test_list_comprehension() {
    check_scope(
        r"
[x*y$0 for x in range(5) for y in range(5)]
        ",
        &["x", "y"],
    )
}

#[test]
fn test_list_comprehension_clause1() {
    check_scope(
        r"
[x*y for x in range(5) for y in range(5) if x*y$0 > 10] 
        ",
        &["x", "y"],
    )
}

#[test]
fn test_list_comprehension_clause2() {
    check_scope(
        r"
[x*y for x in range(5) if x$0yz > 2 for y in range(5) if x*y > 10]
        ",
        &["x"],
    )
}

#[test]
fn test_load() {
    check_scope(
        r#"
load("foo.star", "go_binary")
$0
    "#,
        &["go_binary"],
    )
}

#[test]
fn test_param_defaults() {
    check_scope(
        r#"
_tsc = ""

def ts_project(tsc = _t$0sc):
    pass
    "#,
        &["_tsc"],
    )
}

#[test]
fn test_prelude() {
    check_scope_full(
        r#"
foo = 123
$0   
"#,
        &["bar", "f", "foo"],
        Some(
            r#"
bar = "abc"

def f():
    pass
"#,
        ),
    )
}
