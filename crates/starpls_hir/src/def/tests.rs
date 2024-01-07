use crate::{lower, test_database::TestDatabase, Resolver};
use starpls_common::{parse, File, FileId};
use starpls_test_util::parse_fixture;

fn check_scope(fixture: &str, expected: &[&str]) {
    let test_db: TestDatabase = Default::default();
    let file_id = FileId(0);
    let (text, offset) = parse_fixture(fixture);
    let file = File::new(&test_db, file_id, text);
    let parse = parse(&test_db, file);
    let info = lower(&test_db, parse);
    let resolver = Resolver::new_for_offset(&test_db, info, offset);
    let mut actual = resolver
        .names()
        .keys()
        .map(|name| name.inner(&test_db).as_str())
        .collect::<Vec<_>>();
    actual.sort();
    assert_eq!(expected, &actual[..]);
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
