use crate::{test_database::TestDatabase, Resolver};
use starpls_common::{File, FileId};
use starpls_test_util::parse_fixture;

fn check_scope(fixture: &str, expected: &[&str]) {
    let test_db: TestDatabase = Default::default();
    let file_id = FileId(0);
    let (text, offset) = parse_fixture(fixture);
    let file = File::new(&test_db, file_id, text);
    let resolver = Resolver::new_for_offset(&test_db, file, offset);
    let names = resolver.module_names();
    let mut actual = names.keys().map(|name| name.as_str()).collect::<Vec<_>>();
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
[x*y for x in range(5) if x$0 > 2 for y in range(5) if x*y > 10] 
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
