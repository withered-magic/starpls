use crate::{source_map, test_database::TestDatabase, Db as _, DisplayWithDb};
use expect_test::{expect, Expect};
use itertools::Itertools;
use starpls_common::{parse, Db as _, FileId};
use starpls_syntax::ast::AstNode;
use std::{cmp::Ordering, fmt::Write};

fn check_infer(input: &str, expect: Expect) {
    let mut db = TestDatabase::default();
    let file_id = FileId(0);
    let file = db.set_file_contents(file_id, input.to_string());
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

    let diagnostics = db.gcx.with_tcx(&db, |tcx| tcx.diagnostics_for_file(file));
    if !diagnostics.is_empty() {
        res.push('\n');
        for diagnostic in diagnostics.iter() {
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
