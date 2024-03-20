use crate::{source_map, test_database::TestDatabase, Db as _, DisplayWithDb};
use expect_test::{expect, Expect};
use itertools::Itertools;
use starpls_common::{parse, Db as _, Dialect, FileId};
use starpls_syntax::ast::AstNode;
use std::{cmp::Ordering, fmt::Write};

fn check_infer(input: &str, expect: Expect) {
    let mut db = TestDatabase::default();
    let file_id = FileId(0);
    let file = db.create_file(file_id, Dialect::Standard, input.to_string());
    let root = parse(&db, file).syntax(&db);
    let source_map = source_map(&db, file);
    let mut res = String::new();

    for (ptr, range) in source_map
        .expr_map
        .keys()
        .map(|ptr| (ptr, ptr.syntax_node_ptr().text_range()))
        .sorted_by(|(_, lhs), (_, rhs)| {
            if lhs.contains_range(rhs.clone()) {
                Ordering::Greater
            } else if rhs.contains_range(lhs.clone()) {
                Ordering::Less
            } else {
                lhs.start().cmp(&rhs.start())
            }
        })
    {
        let expr = *source_map.expr_map.get(&ptr).unwrap();
        let ty = db.infer_expr(file, expr);
        let node = ptr.to_node(&root);
        writeln!(
            res,
            "{:?}..{:?} {:?}: {}",
            range.start(),
            range.end(),
            node.syntax().text(),
            ty.display(&db)
        )
        .unwrap();
    }

    for (ptr, _) in source_map
        .param_map
        .keys()
        .map(|ptr| (ptr, ptr.syntax_node_ptr().text_range()))
        .sorted_by(|(_, lhs), (_, rhs)| {
            if lhs.contains_range(rhs.clone()) {
                Ordering::Greater
            } else if rhs.contains_range(lhs.clone()) {
                Ordering::Less
            } else {
                lhs.start().cmp(&rhs.start())
            }
        })
    {
        let param = *source_map.param_map.get(&ptr).unwrap();
        db.infer_param(file, param);
    }

    let diagnostics = db.gcx.with_tcx(&db, |tcx| tcx.diagnostics_for_file(file));
    if !diagnostics.is_empty() {
        res.push('\n');
        for diagnostic in diagnostics
            .into_iter()
            .sorted_by(|lhs, rhs| lhs.range.range.start().cmp(&rhs.range.range.start()))
        {
            writeln!(
                res,
                "{:?}..{:?} {}",
                diagnostic.range.range.start(),
                diagnostic.range.range.end(),
                diagnostic.message
            )
            .unwrap();
        }
    }

    expect.assert_eq(&res);
}

#[test]
fn test_infer_basic_exprs() {
    check_infer(
        r#"None
True
False
0
0.
"hello"
b"hello"
["foo", "bar"]
(1, 2, 3)
{"a": 1}
"#,
        expect![[r#"
            0..4 "None": None
            5..9 "True": bool
            10..15 "False": bool
            16..17 "0": int
            18..20 "0.": float
            21..28 "\"hello\"": string
            29..37 "b\"hello\"": bytes
            39..44 "\"foo\"": string
            46..51 "\"bar\"": string
            38..52 "[\"foo\", \"bar\"]": list[string]
            54..55 "1": int
            57..58 "2": int
            60..61 "3": int
            53..62 "(1, 2, 3)": tuple[int, int, int]
            64..67 "\"a\"": string
            69..70 "1": int
            63..71 "{\"a\": 1}": dict[string, int]
        "#]],
    );
}

#[test]
fn test_infer_assign_stmt() {
    check_infer(
        r#"
a = 1
b, c = 2, 3
(d) = True
([e, f], g) = ((1, "a"), 3)
h, i = [4, 5, 6]
"#,
        expect![[r#"
            1..2 "a": int
            5..6 "1": int
            7..8 "b": int
            10..11 "c": int
            7..11 "b, c": tuple[int, int]
            14..15 "2": int
            17..18 "3": int
            14..18 "2, 3": tuple[int, int]
            20..21 "d": bool
            19..22 "(d)": Any
            25..29 "True": bool
            32..33 "e": int
            35..36 "f": string
            31..37 "[e, f]": list[Unknown]
            39..40 "g": int
            30..41 "([e, f], g)": tuple[list[Unknown], int]
            46..47 "1": int
            49..52 "\"a\"": string
            45..53 "(1, \"a\")": tuple[int, string]
            55..56 "3": int
            44..57 "((1, \"a\"), 3)": tuple[tuple[int, string], int]
            58..59 "h": int
            61..62 "i": int
            58..62 "h, i": tuple[int, int]
            66..67 "4": int
            69..70 "5": int
            72..73 "6": int
            65..74 "[4, 5, 6]": list[int]
        "#]],
    );
}

#[test]
fn test_common_type() {
    check_infer(
        r#"
[]
[1, 2]
[1, "a"]
{}
{"a": 1}
{"a": 1, "b": "c"}
{"a": 1, 1: "a"}
"#,
        expect![[r#"
            1..3 "[]": list[Unknown]
            5..6 "1": int
            8..9 "2": int
            4..10 "[1, 2]": list[int]
            12..13 "1": int
            15..18 "\"a\"": string
            11..19 "[1, \"a\"]": list[Unknown]
            20..22 "{}": dict[Any, Unknown]
            24..27 "\"a\"": string
            29..30 "1": int
            23..31 "{\"a\": 1}": dict[string, int]
            33..36 "\"a\"": string
            38..39 "1": int
            41..44 "\"b\"": string
            46..49 "\"c\"": string
            32..50 "{\"a\": 1, \"b\": \"c\"}": dict[string, Unknown]
            52..55 "\"a\"": string
            57..58 "1": int
            60..61 "1": int
            63..66 "\"a\"": string
            51..67 "{\"a\": 1, 1: \"a\"}": dict[Any, Unknown]
        "#]],
    );
}

#[test]
fn test_bad_assign_type_comment() {
    check_infer(
        r#"
greeting = 1 # type: string
    "#,
        expect![[r#"
            1..9 "greeting": string
            12..13 "1": int

            12..13 Expected value of type "string"
        "#]],
    )
}

#[test]
fn test_type_ignore_comment() {
    check_infer(
        r#"
res1 = 1 + "x"
res2 = 2 + "y" # type: ignore
    "#,
        expect![[r#"
            1..5 "res1": Unknown
            8..9 "1": int
            12..15 "\"x\"": string
            8..15 "1 + \"x\"": Unknown
            16..20 "res2": Unknown
            23..24 "2": int
            27..30 "\"y\"": string
            23..30 "2 + \"y\"": Unknown

            8..15 Operator "+" not supported for types "int" and "string"
        "#]],
    )
}

#[test]
fn test_invalid_type_refs() {
    check_infer(
        r#"
num = 1 # type: foo

def frobnicate(
    x, # type: bar
):
    pass
"#,
        expect![[r#"
            1..4 "num": int
            7..8 "1": int

            9..20 Unknown type "foo"
            42..43 Unknown type "bar"
        "#]],
    )
}

#[test]
fn test_union() {
    check_infer(
        r#"
def foo(x):
    # type: (int) -> None
    pass

def bar(x):
    # type: (int | string | None) -> None
    pass

x = 1 # type: int | None
foo(x)
bar(x)
bar(2)

y = "hello" # type: int | string
bar(y)

y = "goodbye" # type: int | string | float | None
bar(y)
"#,
        expect![[r#"
            113..114 "x": int | None
            117..118 "1": int
            138..141 "foo": def foo(x: int) -> None
            142..143 "x": int | None
            138..144 "foo(x)": None
            145..148 "bar": def bar(x: int | string | None) -> None
            149..150 "x": int | None
            145..151 "bar(x)": None
            152..155 "bar": def bar(x: int | string | None) -> None
            156..157 "2": int
            152..158 "bar(2)": None
            160..161 "y": int | string
            164..171 "\"hello\"": string
            193..196 "bar": def bar(x: int | string | None) -> None
            197..198 "y": int | string
            193..199 "bar(y)": None
            201..202 "y": int | string | float | None
            205..214 "\"goodbye\"": string
            251..254 "bar": def bar(x: int | string | None) -> None
            255..256 "y": int | string | float | None
            251..257 "bar(y)": None

            255..256 Argument of type "int | string | float | None" cannot be assigned to parameter of type "int | string | None"
        "#]],
    )
}

#[test]
fn test_call_full() {
    check_infer(
        r#"
def foo(a, b, *args, d, **kwargs):
    pass

foo(1, 2, 3, 4, d=5, e=6)
"#,
        expect![[r#"
            46..49 "foo": def foo(a: Unknown, b: Unknown, *args: tuple[Unknown, ...], d: Unknown, **kwargs: dict[string, Unknown]) -> Unknown
            50..51 "1": int
            53..54 "2": int
            56..57 "3": int
            59..60 "4": int
            64..65 "5": int
            69..70 "6": int
            46..71 "foo(1, 2, 3, 4, d=5, e=6)": Unknown
        "#]],
    );
}

#[test]
fn test_call_varargs_kwargs() {
    check_infer(
        r#"
def foo(*args, **kwargs):
    pass

foo(1, 2, a=3, b=4)
"#,
        expect![[r#"
            37..40 "foo": def foo(*args: tuple[Unknown, ...], **kwargs: dict[string, Unknown]) -> Unknown
            41..42 "1": int
            44..45 "2": int
            49..50 "3": int
            54..55 "4": int
            37..56 "foo(1, 2, a=3, b=4)": Unknown
        "#]],
    );
}

#[test]
fn test_call_unexpected_argument() {
    check_infer(
        r#"
def foo(bar):
    pass

foo(1)
foo(baz=1)
    "#,
        expect![[r#"
            25..28 "foo": def foo(bar: Unknown) -> Unknown
            29..30 "1": int
            25..31 "foo(1)": Unknown
            32..35 "foo": def foo(bar: Unknown) -> Unknown
            40..41 "1": int
            32..42 "foo(baz=1)": Unknown

            32..42 Argument missing for parameter(s) "bar"
            40..41 Unexpected keyword argument "baz"
        "#]],
    );
}

#[test]
fn test_call_keyword_only() {
    check_infer(
        r#"
def foo(*, bar):
    pass

foo(1)
foo(2, bar=3)
foo(bar=4)
"#,
        expect![[r#"
            28..31 "foo": def foo(*, bar: Unknown) -> Unknown
            32..33 "1": int
            28..34 "foo(1)": Unknown
            35..38 "foo": def foo(*, bar: Unknown) -> Unknown
            39..40 "2": int
            46..47 "3": int
            35..48 "foo(2, bar=3)": Unknown
            49..52 "foo": def foo(*, bar: Unknown) -> Unknown
            57..58 "4": int
            49..59 "foo(bar=4)": Unknown

            28..34 Argument missing for parameter(s) "bar"
            32..33 Unexpected positional argument
            39..40 Unexpected positional argument
        "#]],
    );
}

#[test]
fn test_call_redundant_kwargs() {
    check_infer(
        r#"
def foo(bar):
    pass

foo(bar=1, bar=2)
"#,
        expect![[r#"
            25..28 "foo": def foo(bar: Unknown) -> Unknown
            33..34 "1": int
            40..41 "2": int
            25..42 "foo(bar=1, bar=2)": Unknown

            40..41 Unexpected keyword argument "bar"
        "#]],
    );
}
