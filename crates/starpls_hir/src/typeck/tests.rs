use std::cmp::Ordering;
use std::fmt::Write;

use expect_test::expect;
use expect_test::Expect;
use itertools::Itertools;
use starpls_bazel::APIContext;
use starpls_common::parse;
use starpls_common::Db as _;
use starpls_common::Dialect;
use starpls_common::FileId;
use starpls_common::FileInfo;
use starpls_syntax::ast::AstNode;
use starpls_test_util::FixtureType;

use crate::source_map;
use crate::test_database::TestDatabaseBuilder;
use crate::Db as _;
use crate::DisplayWithDb;
use crate::InferenceOptions;

fn check_infer(input: &str, expect: Expect) {
    check_infer_with_options(
        input,
        expect,
        InferenceOptions {
            allow_unused_definitions: true,
            ..Default::default()
        },
    )
}

fn check_infer_with_code_flow_analysis(input: &str, expect: Expect) {
    check_infer_with_options(
        input,
        expect,
        InferenceOptions {
            use_code_flow_analysis: true,
            allow_unused_definitions: true,
            ..Default::default()
        },
    )
}

fn check_infer_with_unused_definitions(input: &str, expect: Expect) {
    check_infer_with_options(input, expect, Default::default())
}

fn check_infer_with_options(input: &str, expect: Expect, options: InferenceOptions) {
    let mut builder = TestDatabaseBuilder::default();
    builder.add_function("provider");
    builder.add_function("rule");
    builder.add_function("struct");
    builder.add_type(FixtureType::new("File", vec![], vec![]));
    builder.add_type(FixtureType::new(
        "ctx",
        vec![
            ("attr", "struct"),
            ("executable", "struct"),
            ("file", "struct"),
            ("files", "struct"),
            ("outputs", "struct"),
        ],
        vec![],
    ));
    builder.add_type(FixtureType::new(
        "repository_ctx",
        vec![("attr", "struct")],
        vec![],
    ));
    builder.add_type(FixtureType::new(
        "DefaultInfo",
        vec![("file", "string")],
        vec![],
    ));
    builder.add_type(FixtureType::new(
        "FeatureFlagInfo",
        vec![("value", "string")],
        vec![],
    ));
    builder.add_type(FixtureType::new(
        "config_common",
        vec![],
        vec!["FeatureFlagInfo"],
    ));
    builder.add_function("DefaultInfo");
    builder.add_type(FixtureType::new(
        "PyInfo",
        vec![("field1", "string")],
        vec![],
    ));
    builder.add_type(FixtureType::new("CcInfo", vec![], vec![]));
    builder.add_type(FixtureType::new(
        "attr",
        vec![],
        vec![
            "bool",
            "int",
            "int_list",
            "label",
            "label_keyed_string_dict",
            "label_list",
            "output",
            "output_list",
            "string",
            "string_dict",
            "string_keyed_label_dict",
            "string_list",
            "string_list_dict",
        ],
    ));
    builder.add_global("attr", "attr");
    builder.add_global("config_common", "config_common");
    builder.add_global("PyInfo", "PyInfo");
    builder.set_inference_options(options);

    let mut db = builder.build();
    let file_id = FileId(0);
    let file = db.create_file(
        file_id,
        Dialect::Bazel,
        Some(FileInfo::Bazel {
            api_context: APIContext::Bzl,
            is_external: false,
        }),
        input.to_string(),
    );
    let root = parse(&db, file).syntax(&db);
    let source_map = source_map(&db, file);
    let mut res = String::new();

    for (ptr, range) in source_map
        .expr_map
        .keys()
        .map(|ptr| (ptr, ptr.syntax_node_ptr().text_range()))
        .sorted_by(|(_, lhs), (_, rhs)| {
            if lhs.contains_range(*rhs) {
                Ordering::Greater
            } else if rhs.contains_range(*lhs) {
                Ordering::Less
            } else {
                lhs.start().cmp(&rhs.start())
            }
        })
    {
        let expr = *source_map.expr_map.get(ptr).unwrap();
        let ty = db.gcx().with_tcx(&db, |tcx| tcx.infer_expr(file, expr));
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
            if lhs.contains_range(*rhs) {
                Ordering::Greater
            } else if rhs.contains_range(*lhs) {
                Ordering::Less
            } else {
                lhs.start().cmp(&rhs.start())
            }
        })
    {
        let param = *source_map.param_map.get(ptr).unwrap();
        db.gcx().with_tcx(&db, |tcx| {
            tcx.infer_param(file, param);
        });
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
            5..9 "True": Literal[True]
            10..15 "False": Literal[False]
            16..17 "0": Literal[0]
            18..20 "0.": float
            21..28 "\"hello\"": Literal["hello"]
            29..37 "b\"hello\"": bytes
            39..44 "\"foo\"": Literal["foo"]
            46..51 "\"bar\"": Literal["bar"]
            38..52 "[\"foo\", \"bar\"]": list[string]
            54..55 "1": Literal[1]
            57..58 "2": Literal[2]
            60..61 "3": Literal[3]
            53..62 "(1, 2, 3)": tuple[Literal[1], Literal[2], Literal[3]]
            64..67 "\"a\"": Literal["a"]
            69..70 "1": Literal[1]
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
            1..2 "a": Literal[1]
            5..6 "1": Literal[1]
            7..8 "b": Literal[2]
            10..11 "c": Literal[3]
            7..11 "b, c": tuple[Literal[2], Literal[3]]
            14..15 "2": Literal[2]
            17..18 "3": Literal[3]
            14..18 "2, 3": tuple[Literal[2], Literal[3]]
            20..21 "d": Literal[True]
            19..22 "(d)": Literal[True]
            25..29 "True": Literal[True]
            32..33 "e": Literal[1]
            35..36 "f": Literal["a"]
            31..37 "[e, f]": list[Unknown]
            39..40 "g": Literal[3]
            30..41 "([e, f], g)": tuple[list[Unknown], Literal[3]]
            46..47 "1": Literal[1]
            49..52 "\"a\"": Literal["a"]
            45..53 "(1, \"a\")": tuple[Literal[1], Literal["a"]]
            55..56 "3": Literal[3]
            44..57 "((1, \"a\"), 3)": tuple[tuple[Literal[1], Literal["a"]], Literal[3]]
            58..59 "h": int
            61..62 "i": int
            58..62 "h, i": tuple[int, int]
            66..67 "4": Literal[4]
            69..70 "5": Literal[5]
            72..73 "6": Literal[6]
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
            5..6 "1": Literal[1]
            8..9 "2": Literal[2]
            4..10 "[1, 2]": list[int]
            12..13 "1": Literal[1]
            15..18 "\"a\"": Literal["a"]
            11..19 "[1, \"a\"]": list[Unknown]
            20..22 "{}": dict[Unknown, Unknown]
            24..27 "\"a\"": Literal["a"]
            29..30 "1": Literal[1]
            23..31 "{\"a\": 1}": dict[string, int]
            33..36 "\"a\"": Literal["a"]
            38..39 "1": Literal[1]
            41..44 "\"b\"": Literal["b"]
            46..49 "\"c\"": Literal["c"]
            32..50 "{\"a\": 1, \"b\": \"c\"}": dict[string, Unknown]
            52..55 "\"a\"": Literal["a"]
            57..58 "1": Literal[1]
            60..61 "1": Literal[1]
            63..66 "\"a\"": Literal["a"]
            51..67 "{\"a\": 1, 1: \"a\"}": dict[string | int, Unknown]
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
            12..13 "1": Literal[1]

            12..13 Expression of type "Literal[1]" cannot be assigned to variable of type "string"
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
            8..9 "1": Literal[1]
            12..15 "\"x\"": Literal["x"]
            8..15 "1 + \"x\"": Unknown
            16..20 "res2": Unknown
            23..24 "2": Literal[2]
            27..30 "\"y\"": Literal["y"]
            23..30 "2 + \"y\"": Unknown

            8..15 Operator "+" not supported for types "Literal[1]" and "Literal["x"]"
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
            1..4 "num": Literal[1]
            7..8 "1": Literal[1]

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
            117..118 "1": Literal[1]
            138..141 "foo": def foo(x: int) -> None
            142..143 "x": int | None
            138..144 "foo(x)": None
            145..148 "bar": def bar(x: int | string | None) -> None
            149..150 "x": int | None
            145..151 "bar(x)": None
            152..155 "bar": def bar(x: int | string | None) -> None
            156..157 "2": Literal[2]
            152..158 "bar(2)": None
            160..161 "y": int | string
            164..171 "\"hello\"": Literal["hello"]
            193..196 "bar": def bar(x: int | string | None) -> None
            197..198 "y": int | string
            193..199 "bar(y)": None
            201..202 "y": int | string | float | None
            205..214 "\"goodbye\"": Literal["goodbye"]
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
            46..49 "foo": def foo(a, b, *args: Unknown, d, **kwargs: Unknown) -> Unknown
            50..51 "1": Literal[1]
            53..54 "2": Literal[2]
            56..57 "3": Literal[3]
            59..60 "4": Literal[4]
            64..65 "5": Literal[5]
            69..70 "6": Literal[6]
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
            37..40 "foo": def foo(*args: Unknown, **kwargs: Unknown) -> Unknown
            41..42 "1": Literal[1]
            44..45 "2": Literal[2]
            49..50 "3": Literal[3]
            54..55 "4": Literal[4]
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
            25..28 "foo": def foo(bar) -> Unknown
            29..30 "1": Literal[1]
            25..31 "foo(1)": Unknown
            32..35 "foo": def foo(bar) -> Unknown
            40..41 "1": Literal[1]
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
            28..31 "foo": def foo(*, bar) -> Unknown
            32..33 "1": Literal[1]
            28..34 "foo(1)": Unknown
            35..38 "foo": def foo(*, bar) -> Unknown
            39..40 "2": Literal[2]
            46..47 "3": Literal[3]
            35..48 "foo(2, bar=3)": Unknown
            49..52 "foo": def foo(*, bar) -> Unknown
            57..58 "4": Literal[4]
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
            25..28 "foo": def foo(bar) -> Unknown
            33..34 "1": Literal[1]
            40..41 "2": Literal[2]
            25..42 "foo(bar=1, bar=2)": Unknown

            40..41 Unexpected keyword argument "bar"
        "#]],
    );
}

#[test]
fn test_call_expr_arg_order() {
    check_infer(
        r#"
def foo(x, y):
    pass

args = []
kwargs = {}
foo(y=1, 2)
foo(**kwargs, 2)
foo(y=1, *args)
foo(**kwargs, *args)
"#,
        expect![[r#"
            26..30 "args": list[Unknown]
            33..35 "[]": list[Unknown]
            36..42 "kwargs": dict[Unknown, Unknown]
            45..47 "{}": dict[Unknown, Unknown]
            48..51 "foo": def foo(x, y) -> Unknown
            54..55 "1": Literal[1]
            57..58 "2": Literal[2]
            48..59 "foo(y=1, 2)": Unknown
            60..63 "foo": def foo(x, y) -> Unknown
            66..72 "kwargs": dict[Unknown, Unknown]
            74..75 "2": Literal[2]
            60..76 "foo(**kwargs, 2)": Unknown
            77..80 "foo": def foo(x, y) -> Unknown
            83..84 "1": Literal[1]
            87..91 "args": list[Unknown]
            77..92 "foo(y=1, *args)": Unknown
            93..96 "foo": def foo(x, y) -> Unknown
            99..105 "kwargs": dict[Unknown, Unknown]
            108..112 "args": list[Unknown]
            93..113 "foo(**kwargs, *args)": Unknown

            57..58 Positional argument cannot follow keyword arguments
            74..75 Positional argument cannot follow keyword argument unpacking
            83..84 Unexpected keyword argument "y"
            87..91 Unpacked iterable argument cannot follow keyword arguments
            108..112 Unpacked iterable argument cannot follow keyword argument unpacking
        "#]],
    );
}

#[test]
fn test_dict_constructor() {
    check_infer(
        r#"
foo = dict(a = 1, b = 2, c = 3)
bar = dict(d = 4, e = "five", f = 6.)
baz = dict()
"#,
        expect![[r#"
            1..4 "foo": dict[string, Unknown]
            7..11 "dict": def dict(x0: dict[Unknown, Unknown] | Iterable[Iterable[Any]] = None, **kwargs) -> dict[Unknown, Unknown]
            16..17 "1": Literal[1]
            23..24 "2": Literal[2]
            30..31 "3": Literal[3]
            7..32 "dict(a = 1, b = 2, c = 3)": dict[string, Unknown]
            33..36 "bar": dict[string, Unknown]
            39..43 "dict": def dict(x0: dict[Unknown, Unknown] | Iterable[Iterable[Any]] = None, **kwargs) -> dict[Unknown, Unknown]
            48..49 "4": Literal[4]
            55..61 "\"five\"": Literal["five"]
            67..69 "6.": float
            39..70 "dict(d = 4, e = \"five\", f = 6.)": dict[string, Unknown]
            71..74 "baz": dict[Unknown, Unknown]
            77..81 "dict": def dict(x0: dict[Unknown, Unknown] | Iterable[Iterable[Any]] = None, **kwargs) -> dict[Unknown, Unknown]
            77..83 "dict()": dict[Unknown, Unknown]
        "#]],
    )
}

#[test]
fn test_dict_union() {
    check_infer(
        r#"
a = 1 # type: int | string | int
x = {"x": 1}
y = {1: "x"}
z = x | y
"#,
        expect![[r#"
            1..2 "a": int | string
            5..6 "1": Literal[1]
            34..35 "x": dict[string, int]
            39..42 "\"x\"": Literal["x"]
            44..45 "1": Literal[1]
            38..46 "{\"x\": 1}": dict[string, int]
            47..48 "y": dict[int, string]
            52..53 "1": Literal[1]
            55..58 "\"x\"": Literal["x"]
            51..59 "{1: \"x\"}": dict[int, string]
            60..61 "z": dict[string | int, int | string]
            64..65 "x": dict[string, int]
            68..69 "y": dict[int, string]
            64..69 "x | y": dict[string | int, int | string]
        "#]],
    )
}

#[test]
fn test_list_addition() {
    check_infer(
        r#"
a = [1] + [2]
x = [1, 2, 3] + ["a", "b", "c"]    
y = x[0]
i = 1 # type: int | string
j = [i] + [""]
"#,
        expect![[r#"
            1..2 "a": list[int]
            6..7 "1": Literal[1]
            5..8 "[1]": list[int]
            12..13 "2": Literal[2]
            11..14 "[2]": list[int]
            5..14 "[1] + [2]": list[int]
            15..16 "x": list[int | string]
            20..21 "1": Literal[1]
            23..24 "2": Literal[2]
            26..27 "3": Literal[3]
            19..28 "[1, 2, 3]": list[int]
            32..35 "\"a\"": Literal["a"]
            37..40 "\"b\"": Literal["b"]
            42..45 "\"c\"": Literal["c"]
            31..46 "[\"a\", \"b\", \"c\"]": list[string]
            19..46 "[1, 2, 3] + [\"a\", \"b\", \"c\"]": list[int | string]
            51..52 "y": int | string
            55..56 "x": list[int | string]
            57..58 "0": Literal[0]
            55..59 "x[0]": int | string
            60..61 "i": int | string
            64..65 "1": Literal[1]
            87..88 "j": list[int | string]
            92..93 "i": int | string
            91..94 "[i]": list[int | string]
            98..100 "\"\"": Literal[""]
            97..101 "[\"\"]": list[string]
            91..101 "[i] + [\"\"]": list[int | string]
        "#]],
    )
}

#[test]
fn test_sequence_repetition() {
    check_infer(
        r#"
"abc" * 3
3 * "abc"
b"abc" * 3
3 * b"abc"
[1] * 3
3 * [1]
x = (1, "") # type: tuple[int, string]
x * 3
3 * x
y = (1, "") # type: tuple[int | string, ...]
y * 3
3 * y
"#,
        expect![[r#"
            1..6 "\"abc\"": Literal["abc"]
            9..10 "3": Literal[3]
            1..10 "\"abc\" * 3": string
            11..12 "3": Literal[3]
            15..20 "\"abc\"": Literal["abc"]
            11..20 "3 * \"abc\"": string
            21..27 "b\"abc\"": bytes
            30..31 "3": Literal[3]
            21..31 "b\"abc\" * 3": bytes
            32..33 "3": Literal[3]
            36..42 "b\"abc\"": bytes
            32..42 "3 * b\"abc\"": bytes
            44..45 "1": Literal[1]
            43..46 "[1]": list[int]
            49..50 "3": Literal[3]
            43..50 "[1] * 3": list[int]
            51..52 "3": Literal[3]
            56..57 "1": Literal[1]
            55..58 "[1]": list[int]
            51..58 "3 * [1]": list[int]
            59..60 "x": tuple[int, string]
            64..65 "1": Literal[1]
            67..69 "\"\"": Literal[""]
            63..70 "(1, \"\")": tuple[Literal[1], Literal[""]]
            98..99 "x": tuple[int, string]
            102..103 "3": Literal[3]
            98..103 "x * 3": tuple[int | string, ...]
            104..105 "3": Literal[3]
            108..109 "x": tuple[int, string]
            104..109 "3 * x": tuple[int | string, ...]
            110..111 "y": tuple[int | string, ...]
            115..116 "1": Literal[1]
            118..120 "\"\"": Literal[""]
            114..121 "(1, \"\")": tuple[Literal[1], Literal[""]]
            155..156 "y": tuple[int | string, ...]
            159..160 "3": Literal[3]
            155..160 "y * 3": tuple[int | string, ...]
            161..162 "3": Literal[3]
            165..166 "y": tuple[int | string, ...]
            161..166 "3 * y": tuple[int | string, ...]
        "#]],
    )
}

#[test]
fn test_struct() {
    check_infer(
        r#"
foo = struct(a = 1, b = "bar")
foo.a
foo.b
foo.c
"#,
        expect![[r#"
            1..4 "foo": struct
            7..13 "struct": def struct(*args, **kwargs) -> Unknown
            18..19 "1": Literal[1]
            25..30 "\"bar\"": Literal["bar"]
            7..31 "struct(a = 1, b = \"bar\")": struct
            32..35 "foo": struct
            32..37 "foo.a": Literal[1]
            38..41 "foo": struct
            38..43 "foo.b": Literal["bar"]
            44..47 "foo": struct
            44..49 "foo.c": Unknown
        "#]],
    )
}

#[test]
fn test_provider() {
    check_infer(
        r#"
DataInfo = provider(
    fields = {
        "foo": "The foo field",
        "bar": "The bar field",
    },
)

info = DataInfo(foo = "foo", bar = "bar")
    "#,
        expect![[r#"
            1..9 "DataInfo": Provider[DataInfo]
            12..20 "provider": def provider(*args, **kwargs) -> Unknown
            45..50 "\"foo\"": Literal["foo"]
            52..67 "\"The foo field\"": Literal["The foo field"]
            77..82 "\"bar\"": Literal["bar"]
            84..99 "\"The bar field\"": Literal["The bar field"]
            35..106 "{\n        \"foo\": \"The foo field\",\n        \"bar\": \"The bar field\",\n    }": dict[string, string]
            12..109 "provider(\n    fields = {\n        \"foo\": \"The foo field\",\n        \"bar\": \"The bar field\",\n    },\n)": Provider[DataInfo]
            111..115 "info": DataInfo
            118..126 "DataInfo": Provider[DataInfo]
            133..138 "\"foo\"": Literal["foo"]
            146..151 "\"bar\"": Literal["bar"]
            118..152 "DataInfo(foo = \"foo\", bar = \"bar\")": DataInfo
        "#]],
    )
}

#[test]
fn test_provider_constructor() {
    check_infer(
        r#"
def validate(*args, **kwargs):
    pass

DataInfo, data_info_ctor = provider(init = validate)
info1 = DataInfo()
info2 = DataInfo()
    "#,
        expect![[r#"
            42..50 "DataInfo": Provider[DataInfo]
            52..66 "data_info_ctor": ProviderRawConstructor
            42..66 "DataInfo, data_info_ctor": tuple[Provider[DataInfo], ProviderRawConstructor]
            69..77 "provider": def provider(*args, **kwargs) -> Unknown
            85..93 "validate": def validate(*args: Unknown, **kwargs: Unknown) -> Unknown
            69..94 "provider(init = validate)": tuple[Provider[DataInfo], ProviderRawConstructor]
            95..100 "info1": DataInfo
            103..111 "DataInfo": Provider[DataInfo]
            103..113 "DataInfo()": DataInfo
            114..119 "info2": DataInfo
            122..130 "DataInfo": Provider[DataInfo]
            122..132 "DataInfo()": DataInfo
        "#]],
    )
}

#[test]
fn test_anonymous_provider() {
    check_infer(
        r#"
providers = struct(DefaultInfo = provider())
info = providers.DefaultInfo()

providers = struct(result = provider(init = None))
info1 = providers.result[0]()
info2 = providers.result[1]()
"#,
        expect![[r#"
            1..10 "providers": struct
            13..19 "struct": def struct(*args, **kwargs) -> Unknown
            34..42 "provider": def provider(*args, **kwargs) -> Unknown
            34..44 "provider()": Provider[_]
            13..45 "struct(DefaultInfo = provider())": struct
            46..50 "info": _
            53..62 "providers": struct
            53..74 "providers.DefaultInfo": Provider[_]
            53..76 "providers.DefaultInfo()": _
            78..87 "providers": struct
            90..96 "struct": def struct(*args, **kwargs) -> Unknown
            106..114 "provider": def provider(*args, **kwargs) -> Unknown
            122..126 "None": None
            106..127 "provider(init = None)": tuple[Provider[_], ProviderRawConstructor]
            90..128 "struct(result = provider(init = None))": struct
            129..134 "info1": _
            137..146 "providers": struct
            137..153 "providers.result": tuple[Provider[_], ProviderRawConstructor]
            154..155 "0": Literal[0]
            137..156 "providers.result[0]": Provider[_]
            137..158 "providers.result[0]()": _
            159..164 "info2": _
            167..176 "providers": struct
            167..183 "providers.result": tuple[Provider[_], ProviderRawConstructor]
            184..185 "1": Literal[1]
            167..186 "providers.result[1]": ProviderRawConstructor
            167..188 "providers.result[1]()": _
        "#]],
    )
}

#[test]
fn test_provider_indexing() {
    check_infer(
        r#"
GoInfo = provider()

def foo(foo, bar, baz):
    # type: (Unknown, Any) -> None
    info1 = foo[GoInfo]
    info2 = bar[GoInfo]
    x1 = foo[0]
    x2 = bar[0]
"#,
        expect![[r#"
            1..7 "GoInfo": Provider[GoInfo]
            10..18 "provider": def provider(*args, **kwargs) -> Unknown
            10..20 "provider()": Provider[GoInfo]
            85..90 "info1": GoInfo
            93..96 "foo": Unknown
            97..103 "GoInfo": Provider[GoInfo]
            93..104 "foo[GoInfo]": GoInfo
            109..114 "info2": GoInfo
            117..120 "bar": Any
            121..127 "GoInfo": Provider[GoInfo]
            117..128 "bar[GoInfo]": GoInfo
            133..135 "x1": Unknown
            138..141 "foo": Unknown
            142..143 "0": Literal[0]
            138..144 "foo[0]": Unknown
            149..151 "x2": Unknown
            154..157 "bar": Any
            158..159 "0": Literal[0]
            154..160 "bar[0]": Unknown
        "#]],
    )
}

#[test]
fn test_provider_type_comments() {
    check_infer(
        r#"
DataInfo = provider(
    fields = {
        "foo": "The foo field",
    },
)

def a():
    return DataInfo(foo = "abc")

def f(info):
    # type: (DataInfo) -> list[DataInfo]
    print(info.foo)
    return [info]

info1 = a() # type: DataInfo
info2 = a() # type: DataInfo | None
infos = f(info1)
"#,
        expect![[r#"
            1..9 "DataInfo": Provider[DataInfo]
            12..20 "provider": def provider(*args, **kwargs) -> Unknown
            45..50 "\"foo\"": Literal["foo"]
            52..67 "\"The foo field\"": Literal["The foo field"]
            35..74 "{\n        \"foo\": \"The foo field\",\n    }": dict[string, string]
            12..77 "provider(\n    fields = {\n        \"foo\": \"The foo field\",\n    },\n)": Provider[DataInfo]
            99..107 "DataInfo": Provider[DataInfo]
            114..119 "\"abc\"": Literal["abc"]
            99..120 "DataInfo(foo = \"abc\")": DataInfo
            180..185 "print": def print(*args: Any, str: string = None) -> None
            186..190 "info": DataInfo
            186..194 "info.foo": Unknown
            180..195 "print(info.foo)": None
            208..212 "info": DataInfo
            207..213 "[info]": list[DataInfo]
            215..220 "info1": DataInfo
            223..224 "a": def a() -> Unknown
            223..226 "a()": Unknown
            244..249 "info2": DataInfo | None
            252..253 "a": def a() -> Unknown
            252..255 "a()": Unknown
            280..285 "infos": list[DataInfo]
            288..289 "f": def f(info: DataInfo) -> list[DataInfo]
            290..295 "info1": DataInfo
            288..296 "f(info1)": list[DataInfo]
        "#]],
    );
}

#[test]
fn test_path_type_comments() {
    check_infer(
        r#"
DataInfo = provider()
api = struct(DataInfo = DataInfo)

def foo(info):
    # type: (api.DataInfo) -> api.DataInfo
    return info

res = foo(api.DataInfo())
"#,
        expect![[r#"
            1..9 "DataInfo": Provider[DataInfo]
            12..20 "provider": def provider(*args, **kwargs) -> Unknown
            12..22 "provider()": Provider[DataInfo]
            23..26 "api": struct
            29..35 "struct": def struct(*args, **kwargs) -> Unknown
            47..55 "DataInfo": Provider[DataInfo]
            29..56 "struct(DataInfo = DataInfo)": struct
            127..131 "info": DataInfo
            133..136 "res": DataInfo
            139..142 "foo": def foo(info: DataInfo) -> DataInfo
            143..146 "api": struct
            143..155 "api.DataInfo": Provider[DataInfo]
            143..157 "api.DataInfo()": DataInfo
            139..158 "foo(api.DataInfo())": DataInfo
        "#]],
    );
}

#[test]
fn test_unary_expr() {
    check_infer(
        r#"
a = +1
b = -1
c = ~1
d = +1.
e = -1.
f = ~1.
g = not 3
h = 1 # type: Unknown
i = not h
j = 1 # type: Any
k = not j
l = ~"abc"
m = 1 # type: int | float
n = +m
o = ~m
"#,
        expect![[r#"
            1..2 "a": int
            6..7 "1": Literal[1]
            5..7 "+1": int
            8..9 "b": int
            13..14 "1": Literal[1]
            12..14 "-1": int
            15..16 "c": int
            20..21 "1": Literal[1]
            19..21 "~1": int
            22..23 "d": float
            27..29 "1.": float
            26..29 "+1.": float
            30..31 "e": float
            35..37 "1.": float
            34..37 "-1.": float
            38..39 "f": Unknown
            43..45 "1.": float
            42..45 "~1.": Unknown
            46..47 "g": bool
            54..55 "3": Literal[3]
            50..55 "not 3": bool
            56..57 "h": Unknown
            60..61 "1": Literal[1]
            78..79 "i": bool
            86..87 "h": Unknown
            82..87 "not h": bool
            88..89 "j": Any
            92..93 "1": Literal[1]
            106..107 "k": bool
            114..115 "j": Any
            110..115 "not j": bool
            116..117 "l": Unknown
            121..126 "\"abc\"": Literal["abc"]
            120..126 "~\"abc\"": Unknown
            127..128 "m": int | float
            131..132 "1": Literal[1]
            153..154 "n": int | float
            158..159 "m": int | float
            157..159 "+m": int | float
            160..161 "o": Unknown
            165..166 "m": int | float
            164..166 "~m": Unknown

            42..45 Operator "~" is not supported for type "float"
            120..126 Operator "~" is not supported for type "Literal["abc"]"
            164..166 Operator "~" is not supported for type "int | float"
        "#]],
    );
}

#[test]
fn test_if_expr() {
    check_infer(
        r#"
x = 3 if True else 4
y = 1. if True else ""
"#,
        expect![[r#"
            1..2 "x": int
            5..6 "3": Literal[3]
            10..14 "True": Literal[True]
            20..21 "4": Literal[4]
            5..21 "3 if True else 4": int
            22..23 "y": float | string
            26..28 "1.": float
            32..36 "True": Literal[True]
            42..44 "\"\"": Literal[""]
            26..44 "1. if True else \"\"": float | string
        "#]],
    );
}

#[test]
fn test_sequence_assignments() {
    check_infer(
        r#"
def foo(foo, bar):
    # type: (list[Unknown], Iterable[Unknown]) -> Unknown
    pass

a = [] # type: Sequence[Unknown]
foo(a, a)
foo([], [])
"#,
        expect![[r#"
            88..89 "a": Sequence[Unknown]
            92..94 "[]": list[Unknown]
            121..124 "foo": def foo(foo: list[Unknown], bar: Iterable[Unknown]) -> Unknown
            125..126 "a": Sequence[Unknown]
            128..129 "a": Sequence[Unknown]
            121..130 "foo(a, a)": Unknown
            131..134 "foo": def foo(foo: list[Unknown], bar: Iterable[Unknown]) -> Unknown
            135..137 "[]": list[Unknown]
            139..141 "[]": list[Unknown]
            131..142 "foo([], [])": Unknown
        "#]],
    );
}

#[test]
fn test_tuple_assignments() {
    check_infer(
        r#"
"abc".startswith("a")
"abc".startswith(("abc", "ABC"))
"abc".startswith(("abc", 1))
"#,
        expect![[r#"
            1..6 "\"abc\"": Literal["abc"]
            1..17 "\"abc\".startswith": def startswith(x0: string | tuple[string, ...], x1: int = None, x2: int = None) -> bool
            18..21 "\"a\"": Literal["a"]
            1..22 "\"abc\".startswith(\"a\")": bool
            23..28 "\"abc\"": Literal["abc"]
            23..39 "\"abc\".startswith": def startswith(x0: string | tuple[string, ...], x1: int = None, x2: int = None) -> bool
            41..46 "\"abc\"": Literal["abc"]
            48..53 "\"ABC\"": Literal["ABC"]
            40..54 "(\"abc\", \"ABC\")": tuple[Literal["abc"], Literal["ABC"]]
            23..55 "\"abc\".startswith((\"abc\", \"ABC\"))": bool
            56..61 "\"abc\"": Literal["abc"]
            56..72 "\"abc\".startswith": def startswith(x0: string | tuple[string, ...], x1: int = None, x2: int = None) -> bool
            74..79 "\"abc\"": Literal["abc"]
            81..82 "1": Literal[1]
            73..83 "(\"abc\", 1)": tuple[Literal["abc"], Literal[1]]
            56..84 "\"abc\".startswith((\"abc\", 1))": bool

            73..83 Argument of type "tuple[Literal["abc"], Literal[1]]" cannot be assigned to parameter of type "string | tuple[string, ...]"
        "#]],
    )
}

#[test]
fn test_slice_expr() {
    check_infer(
        r#"
x = [1, 2, 3]
x[:]
x[1:]
x[:1]
x[::2]
x[0:2:2]
x["a":None:x]

a = "abc"[:]
b = b"abc"[:]
c = ("a", 1, [])[:]
d = range(10)[:]
e = [1, 2, 3] # type: Sequence[int]
f = e[:]
g = {}[:]

def foo(*nums):
    # type: (*int) -> None
    nums[:]
"#,
        expect![[r#"
            1..2 "x": list[int]
            6..7 "1": Literal[1]
            9..10 "2": Literal[2]
            12..13 "3": Literal[3]
            5..14 "[1, 2, 3]": list[int]
            15..16 "x": list[int]
            15..19 "x[:]": list[int]
            20..21 "x": list[int]
            22..23 "1": Literal[1]
            20..25 "x[1:]": list[int]
            26..27 "x": list[int]
            29..30 "1": Literal[1]
            26..31 "x[:1]": list[int]
            32..33 "x": list[int]
            36..37 "2": Literal[2]
            32..38 "x[::2]": list[int]
            39..40 "x": list[int]
            41..42 "0": Literal[0]
            43..44 "2": Literal[2]
            45..46 "2": Literal[2]
            39..47 "x[0:2:2]": list[int]
            48..49 "x": list[int]
            50..53 "\"a\"": Literal["a"]
            54..58 "None": None
            59..60 "x": list[int]
            48..61 "x[\"a\":None:x]": list[int]
            63..64 "a": string
            67..72 "\"abc\"": Literal["abc"]
            67..75 "\"abc\"[:]": string
            76..77 "b": bytes
            80..86 "b\"abc\"": bytes
            80..89 "b\"abc\"[:]": bytes
            90..91 "c": string | int | list[Unknown]
            95..98 "\"a\"": Literal["a"]
            100..101 "1": Literal[1]
            103..105 "[]": list[Unknown]
            94..106 "(\"a\", 1, [])": tuple[Literal["a"], Literal[1], list[Unknown]]
            94..109 "(\"a\", 1, [])[:]": string | int | list[Unknown]
            110..111 "d": list[int]
            114..119 "range": def range(x0: int, x1: int = None, x2: int = None) -> range
            120..122 "10": Literal[10]
            114..123 "range(10)": range
            114..126 "range(10)[:]": list[int]
            127..128 "e": Sequence[int]
            132..133 "1": Literal[1]
            135..136 "2": Literal[2]
            138..139 "3": Literal[3]
            131..140 "[1, 2, 3]": list[int]
            163..164 "f": list[int]
            167..168 "e": Sequence[int]
            167..171 "e[:]": list[int]
            172..173 "g": Unknown
            176..178 "{}": dict[Unknown, Unknown]
            176..181 "{}[:]": Unknown
            230..234 "nums": tuple[Unknown, ...]
            230..237 "nums[:]": list[Unknown]

            50..53 `start`, `stop`, and `step` operands must be integers or `None`
            59..60 `start`, `stop`, and `step` operands must be integers or `None`
            176..181 Cannot slice expression of type "dict[Unknown, Unknown]"
        "#]],
    )
}

#[test]
fn test_paren_expr() {
    check_infer(
        r#"
()
(1)
(1,)
(1, 2)
"#,
        expect![[r#"
            1..3 "()": tuple[]
            5..6 "1": Literal[1]
            4..7 "(1)": Literal[1]
            9..10 "1": Literal[1]
            8..12 "(1,)": tuple[Literal[1]]
            14..15 "1": Literal[1]
            17..18 "2": Literal[2]
            13..19 "(1, 2)": tuple[Literal[1], Literal[2]]
        "#]],
    );
}

#[test]
fn test_field_signature_struct() {
    check_infer(
        r#"
def _impl(ctx):
    # type: (ctx) -> Unknown
    ctx.executable.foo
    ctx.file.bar
    ctx.files.baz
    ctx.outputs.qux
"#,
        expect![[r#"
            50..53 "ctx": ctx
            50..64 "ctx.executable": struct
            50..68 "ctx.executable.foo": File
            73..76 "ctx": ctx
            73..81 "ctx.file": struct
            73..85 "ctx.file.bar": File
            90..93 "ctx": ctx
            90..99 "ctx.files": struct
            90..103 "ctx.files.baz": list[File]
            108..111 "ctx": ctx
            108..119 "ctx.outputs": struct
            108..123 "ctx.outputs.qux": File
            "#]],
    );
}

#[test]
fn test_simple_if_stmt() {
    check_infer_with_code_flow_analysis(
        r#"
cond = 1 < 2
def f():
    x = 0
    if cond:
        x = "less"
    x
"#,
        expect![[r#"
            1..5 "cond": bool
            8..9 "1": Literal[1]
            12..13 "2": Literal[2]
            8..13 "1 < 2": bool
            27..28 "x": Literal[0]
            31..32 "0": Literal[0]
            40..44 "cond": bool
            54..55 "x": Literal["less"]
            58..64 "\"less\"": Literal["less"]
            69..70 "x": string | int
        "#]],
    );
}

#[test]
fn test_infer_ctx_attrs() {
    check_infer_with_options(
        r#"
def _rule_impl(ctx):
    foo = ctx.file.foo
    srcs = ctx.attr.srcs

my_rule = rule(
    implementation = _rule_impl,
    attrs = {
        "srcs": attr.label_list(),
    },
)

def _repository_rule_impl(repository_ctx):
    srcs = repository_ctx.attr.srcs

my_rule = repository_rule(
    implementation = _repository_rule_impl,
    attrs = {
        "srcs": attr.label_list(),
    },
)
"#,
        expect![[r#"
            26..29 "foo": File
            32..35 "ctx": ctx
            32..40 "ctx.file": struct
            32..44 "ctx.file.foo": File
            49..53 "srcs": list[Target]
            56..59 "ctx": ctx
            56..64 "ctx.attr": struct
            56..69 "ctx.attr.srcs": list[Target]
            71..78 "my_rule": rule
            81..85 "rule": def rule(*args, **kwargs) -> Unknown
            108..118 "_rule_impl": def _rule_impl(ctx) -> Unknown
            142..148 "\"srcs\"": Literal["srcs"]
            150..154 "attr": attr
            150..165 "attr.label_list": def label_list(*args, **kwargs) -> Unknown
            150..167 "attr.label_list()": Attribute
            132..174 "{\n        \"srcs\": attr.label_list(),\n    }": dict[string, Attribute]
            81..177 "rule(\n    implementation = _rule_impl,\n    attrs = {\n        \"srcs\": attr.label_list(),\n    },\n)": rule
            226..230 "srcs": list[Target]
            233..247 "repository_ctx": repository_ctx
            233..252 "repository_ctx.attr": struct
            233..257 "repository_ctx.attr.srcs": list[Target]
            259..266 "my_rule": repository_rule
            269..284 "repository_rule": def repository_rule(implementation: Unknown, attrs: dict[string, Unknown] | None = None, local: bool = None, environ: Sequence[string] = [], configure: bool = False, remotable: bool = False, doc: string | None = None) -> callable
            307..328 "_repository_rule_impl": def _repository_rule_impl(repository_ctx) -> Unknown
            352..358 "\"srcs\"": Literal["srcs"]
            360..364 "attr": attr
            360..375 "attr.label_list": def label_list(*args, **kwargs) -> Unknown
            360..377 "attr.label_list()": Attribute
            342..384 "{\n        \"srcs\": attr.label_list(),\n    }": dict[string, Attribute]
            269..387 "repository_rule(\n    implementation = _repository_rule_impl,\n    attrs = {\n        \"srcs\": attr.label_list(),\n    },\n)": repository_rule
        "#]],
        InferenceOptions {
            infer_ctx_attributes: true,
            use_code_flow_analysis: true,
            allow_unused_definitions: true,
        },
    );
}

#[test]
fn test_infer_ctx_attrs_all() {
    check_infer_with_options(
        r#"
def _rule_impl(ctx):
    ctx.attr.a
    ctx.attr.b
    ctx.attr.c
    ctx.attr.d
    ctx.attr.e
    ctx.attr.f
    ctx.attr.g
    ctx.attr.h
    ctx.attr.i
    ctx.attr.j
    ctx.attr.k
    ctx.attr.l
    ctx.attr.m

my_rule = rule(
    implementation = _rule_impl,
    attrs = {
        "a": attr.bool(),
        "b": attr.int(),
        "c": attr.int_list(),
        "d": attr.label(),
        "e": attr.label_keyed_string_dict(),
        "f": attr.label_list(),
        "g": attr.output(),
        "h": attr.output_list(),
        "i": attr.string(),
        "j": attr.string_dict(),
        "k": attr.string_keyed_label_dict(),
        "l": attr.string_list(),
        "m": attr.string_list_dict(),
    },
)
"#,
        expect![[r#"
            26..29 "ctx": ctx
            26..34 "ctx.attr": struct
            26..36 "ctx.attr.a": bool
            41..44 "ctx": ctx
            41..49 "ctx.attr": struct
            41..51 "ctx.attr.b": int
            56..59 "ctx": ctx
            56..64 "ctx.attr": struct
            56..66 "ctx.attr.c": list[int]
            71..74 "ctx": ctx
            71..79 "ctx.attr": struct
            71..81 "ctx.attr.d": Target
            86..89 "ctx": ctx
            86..94 "ctx.attr": struct
            86..96 "ctx.attr.e": dict[Target, string]
            101..104 "ctx": ctx
            101..109 "ctx.attr": struct
            101..111 "ctx.attr.f": list[Target]
            116..119 "ctx": ctx
            116..124 "ctx.attr": struct
            116..126 "ctx.attr.g": Unknown
            131..134 "ctx": ctx
            131..139 "ctx.attr": struct
            131..141 "ctx.attr.h": list[Unknown]
            146..149 "ctx": ctx
            146..154 "ctx.attr": struct
            146..156 "ctx.attr.i": string
            161..164 "ctx": ctx
            161..169 "ctx.attr": struct
            161..171 "ctx.attr.j": dict[string, string]
            176..179 "ctx": ctx
            176..184 "ctx.attr": struct
            176..186 "ctx.attr.k": dict[string, Target]
            191..194 "ctx": ctx
            191..199 "ctx.attr": struct
            191..201 "ctx.attr.l": list[string]
            206..209 "ctx": ctx
            206..214 "ctx.attr": struct
            206..216 "ctx.attr.m": dict[string, list[string]]
            218..225 "my_rule": rule
            228..232 "rule": def rule(*args, **kwargs) -> Unknown
            255..265 "_rule_impl": def _rule_impl(ctx) -> Unknown
            289..292 "\"a\"": Literal["a"]
            294..298 "attr": attr
            294..303 "attr.bool": def bool(*args, **kwargs) -> Unknown
            294..305 "attr.bool()": Attribute
            315..318 "\"b\"": Literal["b"]
            320..324 "attr": attr
            320..328 "attr.int": def int(*args, **kwargs) -> Unknown
            320..330 "attr.int()": Attribute
            340..343 "\"c\"": Literal["c"]
            345..349 "attr": attr
            345..358 "attr.int_list": def int_list(*args, **kwargs) -> Unknown
            345..360 "attr.int_list()": Attribute
            370..373 "\"d\"": Literal["d"]
            375..379 "attr": attr
            375..385 "attr.label": def label(*args, **kwargs) -> Unknown
            375..387 "attr.label()": Attribute
            397..400 "\"e\"": Literal["e"]
            402..406 "attr": attr
            402..430 "attr.label_keyed_string_dict": def label_keyed_string_dict(*args, **kwargs) -> Unknown
            402..432 "attr.label_keyed_string_dict()": Attribute
            442..445 "\"f\"": Literal["f"]
            447..451 "attr": attr
            447..462 "attr.label_list": def label_list(*args, **kwargs) -> Unknown
            447..464 "attr.label_list()": Attribute
            474..477 "\"g\"": Literal["g"]
            479..483 "attr": attr
            479..490 "attr.output": def output(*args, **kwargs) -> Unknown
            479..492 "attr.output()": Attribute
            502..505 "\"h\"": Literal["h"]
            507..511 "attr": attr
            507..523 "attr.output_list": def output_list(*args, **kwargs) -> Unknown
            507..525 "attr.output_list()": Attribute
            535..538 "\"i\"": Literal["i"]
            540..544 "attr": attr
            540..551 "attr.string": def string(*args, **kwargs) -> Unknown
            540..553 "attr.string()": Attribute
            563..566 "\"j\"": Literal["j"]
            568..572 "attr": attr
            568..584 "attr.string_dict": def string_dict(*args, **kwargs) -> Unknown
            568..586 "attr.string_dict()": Attribute
            596..599 "\"k\"": Literal["k"]
            601..605 "attr": attr
            601..629 "attr.string_keyed_label_dict": def string_keyed_label_dict(*args, **kwargs) -> Unknown
            601..631 "attr.string_keyed_label_dict()": Attribute
            641..644 "\"l\"": Literal["l"]
            646..650 "attr": attr
            646..662 "attr.string_list": def string_list(*args, **kwargs) -> Unknown
            646..664 "attr.string_list()": Attribute
            674..677 "\"m\"": Literal["m"]
            679..683 "attr": attr
            679..700 "attr.string_list_dict": def string_list_dict(*args, **kwargs) -> Unknown
            679..702 "attr.string_list_dict()": Attribute
            279..709 "{\n        \"a\": attr.bool(),\n        \"b\": attr.int(),\n        \"c\": attr.int_list(),\n        \"d\": attr.label(),\n        \"e\": attr.label_keyed_string_dict(),\n        \"f\": attr.label_list(),\n        \"g\": attr.output(),\n        \"h\": attr.output_list(),\n        \"i\": attr.string(),\n        \"j\": attr.string_dict(),\n        \"k\": attr.string_keyed_label_dict(),\n        \"l\": attr.string_list(),\n        \"m\": attr.string_list_dict(),\n    }": dict[string, Attribute]
            228..712 "rule(\n    implementation = _rule_impl,\n    attrs = {\n        \"a\": attr.bool(),\n        \"b\": attr.int(),\n        \"c\": attr.int_list(),\n        \"d\": attr.label(),\n        \"e\": attr.label_keyed_string_dict(),\n        \"f\": attr.label_list(),\n        \"g\": attr.output(),\n        \"h\": attr.output_list(),\n        \"i\": attr.string(),\n        \"j\": attr.string_dict(),\n        \"k\": attr.string_keyed_label_dict(),\n        \"l\": attr.string_list(),\n        \"m\": attr.string_list_dict(),\n    },\n)": rule
        "#]],
        InferenceOptions {
            infer_ctx_attributes: true,
            use_code_flow_analysis: true,
            allow_unused_definitions: true,
        },
    );
}

#[test]
fn test_infer_ctx_attrs_disabled() {
    check_infer(
        r#"
def _rule_impl(ctx):
    foo = ctx.file.foo
    srcs = ctx.attr.srcs

my_rule = rule(
    implementation = _rule_impl,
    attrs = {
        "srcs": attr.label_list(),
    },
)    
"#,
        expect![[r#"
            26..29 "foo": Unknown
            32..35 "ctx": Unknown
            32..40 "ctx.file": Unknown
            32..44 "ctx.file.foo": Unknown
            49..53 "srcs": Unknown
            56..59 "ctx": Unknown
            56..64 "ctx.attr": Unknown
            56..69 "ctx.attr.srcs": Unknown
            71..78 "my_rule": rule
            81..85 "rule": def rule(*args, **kwargs) -> Unknown
            108..118 "_rule_impl": def _rule_impl(ctx) -> Unknown
            142..148 "\"srcs\"": Literal["srcs"]
            150..154 "attr": attr
            150..165 "attr.label_list": def label_list(*args, **kwargs) -> Unknown
            150..167 "attr.label_list()": Attribute
            132..174 "{\n        \"srcs\": attr.label_list(),\n    }": dict[string, Attribute]
            81..177 "rule(\n    implementation = _rule_impl,\n    attrs = {\n        \"srcs\": attr.label_list(),\n    },\n)": rule
        "#]],
    );
}

#[test]
fn test_rule_attr_typecheck() {
    check_infer(
        r#"
def _foo_impl(ctx):
    pass

foo = rule(
    implementation = _foo_impl,
    attrs = {
        "bar": attr.string(),
        "baz": attr.int(),
    },
)

foo(
    name = "foo",
    bar = "bar",
    baz = "baz",
)
"#,
        expect![[r#"
            31..34 "foo": rule
            37..41 "rule": def rule(*args, **kwargs) -> Unknown
            64..73 "_foo_impl": def _foo_impl(ctx) -> Unknown
            97..102 "\"bar\"": Literal["bar"]
            104..108 "attr": attr
            104..115 "attr.string": def string(*args, **kwargs) -> Unknown
            104..117 "attr.string()": Attribute
            127..132 "\"baz\"": Literal["baz"]
            134..138 "attr": attr
            134..142 "attr.int": def int(*args, **kwargs) -> Unknown
            134..144 "attr.int()": Attribute
            87..151 "{\n        \"bar\": attr.string(),\n        \"baz\": attr.int(),\n    }": dict[string, Attribute]
            37..154 "rule(\n    implementation = _foo_impl,\n    attrs = {\n        \"bar\": attr.string(),\n        \"baz\": attr.int(),\n    },\n)": rule
            156..159 "foo": rule
            172..177 "\"foo\"": Literal["foo"]
            189..194 "\"bar\"": Literal["bar"]
            206..211 "\"baz\"": Literal["baz"]
            156..214 "foo(\n    name = \"foo\",\n    bar = \"bar\",\n    baz = \"baz\",\n)": None

            206..211 Argument of type "Literal["baz"]" cannot be assigned to parameter of type "int"
        "#]],
    );
}

#[test]
fn test_if_elif_stmt() {
    check_infer_with_code_flow_analysis(
        r#"
cond = 1 < 2
def f():
    x = 0
    if cond:
        x = "less"
    elif cond:
        x = 1.
    x
"#,
        expect![[r#"
            1..5 "cond": bool
            8..9 "1": Literal[1]
            12..13 "2": Literal[2]
            8..13 "1 < 2": bool
            27..28 "x": Literal[0]
            31..32 "0": Literal[0]
            40..44 "cond": bool
            54..55 "x": Literal["less"]
            58..64 "\"less\"": Literal["less"]
            74..78 "cond": bool
            88..89 "x": float
            92..94 "1.": float
            99..100 "x": string | float | int
        "#]],
    );
}

#[test]
fn test_builtin_provider() {
    check_infer(
        r#"
default_info = DefaultInfo()
default_info.file    

flag_info = config_common.FeatureFlagInfo()
flag_info.value

py_info = PyInfo()
py_info.field1

def f(info):
    # type: (CcInfo) -> None
    info
"#,
        expect![[r#"
            1..13 "default_info": DefaultInfo
            16..27 "DefaultInfo": Provider[DefaultInfo]
            16..29 "DefaultInfo()": DefaultInfo
            30..42 "default_info": DefaultInfo
            30..47 "default_info.file": string
            53..62 "flag_info": FeatureFlagInfo
            65..78 "config_common": config_common
            65..94 "config_common.FeatureFlagInfo": Provider[FeatureFlagInfo]
            65..96 "config_common.FeatureFlagInfo()": FeatureFlagInfo
            97..106 "flag_info": FeatureFlagInfo
            97..112 "flag_info.value": string
            114..121 "py_info": PyInfo
            124..130 "PyInfo": Provider[PyInfo]
            124..132 "PyInfo()": PyInfo
            133..140 "py_info": PyInfo
            133..147 "py_info.field1": string
            195..199 "info": CcInfo
        "#]],
    );
}

#[test]
fn test_tuple_type_comments() {
    check_infer(
        r#"
a = (1, 2) # type: tuple[int, ...]
b = (1, 2) # type: ...
c = (1, 2) # type: list[...]
d = (1, 2) # type: tuple[...]
e = (1, 2) # type: tuple[int, ..., int]
"#,
        expect![[r#"
            1..2 "a": tuple[int, ...]
            6..7 "1": Literal[1]
            9..10 "2": Literal[2]
            5..11 "(1, 2)": tuple[Literal[1], Literal[2]]
            36..37 "b": tuple[Literal[1], Literal[2]]
            41..42 "1": Literal[1]
            44..45 "2": Literal[2]
            40..46 "(1, 2)": tuple[Literal[1], Literal[2]]
            59..60 "c": tuple[Literal[1], Literal[2]]
            64..65 "1": Literal[1]
            67..68 "2": Literal[2]
            63..69 "(1, 2)": tuple[Literal[1], Literal[2]]
            88..89 "d": tuple[Literal[1], Literal[2]]
            93..94 "1": Literal[1]
            96..97 "2": Literal[2]
            92..98 "(1, 2)": tuple[Literal[1], Literal[2]]
            118..119 "e": tuple[Literal[1], Literal[2]]
            123..124 "1": Literal[1]
            126..127 "2": Literal[2]
            122..128 "(1, 2)": tuple[Literal[1], Literal[2]]

            47..58 "..." is not allowed in this context
            70..87 "..." is not allowed in this context
            99..117 "..." is not allowed in this context
            129..157 "..." is not allowed in this context
        "#]],
    );
}

#[test]
fn test_logic_operators() {
    check_infer(
        r#"
x = 3 # type: int
greeting = "hello" # type: string

True or False
False or []
0 or []
None or []
() or []
True or greeting
x or greeting

True and False
False and []
0 and []
None and []
() and []
True and greeting
x and greeting
"#,
        expect![[r#"
            1..2 "x": int
            5..6 "3": Literal[3]
            19..27 "greeting": string
            30..37 "\"hello\"": Literal["hello"]
            54..58 "True": Literal[True]
            62..67 "False": Literal[False]
            54..67 "True or False": Literal[True]
            68..73 "False": Literal[False]
            77..79 "[]": list[Unknown]
            68..79 "False or []": list[Unknown]
            80..81 "0": Literal[0]
            85..87 "[]": list[Unknown]
            80..87 "0 or []": list[Unknown]
            88..92 "None": None
            96..98 "[]": list[Unknown]
            88..98 "None or []": list[Unknown]
            99..101 "()": tuple[]
            105..107 "[]": list[Unknown]
            99..107 "() or []": list[Unknown]
            108..112 "True": Literal[True]
            116..124 "greeting": string
            108..124 "True or greeting": bool | string
            125..126 "x": int
            130..138 "greeting": string
            125..138 "x or greeting": int | string
            140..144 "True": Literal[True]
            149..154 "False": Literal[False]
            140..154 "True and False": Literal[False]
            155..160 "False": Literal[False]
            165..167 "[]": list[Unknown]
            155..167 "False and []": Literal[False]
            168..169 "0": Literal[0]
            174..176 "[]": list[Unknown]
            168..176 "0 and []": Literal[0]
            177..181 "None": None
            186..188 "[]": list[Unknown]
            177..188 "None and []": None
            189..191 "()": tuple[]
            196..198 "[]": list[Unknown]
            189..198 "() and []": tuple[]
            199..203 "True": Literal[True]
            208..216 "greeting": string
            199..216 "True and greeting": bool | string
            217..218 "x": int
            223..231 "greeting": string
            217..231 "x and greeting": int | string
        "#]],
    );
}

#[test]
fn test_lambda() {
    check_infer(
        r#"
lambda x, y, z: x + y + z
"#,
        expect![[r#"
            17..18 "x": Unknown
            21..22 "y": Unknown
            17..22 "x + y": Unknown
            25..26 "z": Unknown
            17..26 "x + y + z": Unknown
            1..26 "lambda x, y, z: x + y + z": def lambda(x, y, z) -> Unknown
        "#]],
    );
}

#[test]
fn test_bad_assignments() {
    check_infer(
        r#"
attr.string = 123
"foo".capitalize = 123
d = {} # type: dict[string, string]
d["foo"] = 1
"#,
        expect![[r#"
            1..5 "attr": attr
            1..12 "attr.string": def string(*args, **kwargs) -> Unknown
            15..18 "123": Literal[123]
            19..24 "\"foo\"": Literal["foo"]
            19..35 "\"foo\".capitalize": def capitalize() -> string
            38..41 "123": Literal[123]
            42..43 "d": dict[string, string]
            46..48 "{}": dict[Unknown, Unknown]
            78..79 "d": dict[string, string]
            80..85 "\"foo\"": Literal["foo"]
            78..86 "d[\"foo\"]": string
            89..90 "1": Literal[1]

            1..12 Cannot assign to field "string" for immutable type "attr"
            19..35 Cannot reassign to method "capitalize" of type "Literal["foo"]"
            78..86 Cannot use value of type "Literal[1]" as type "string" in assignment
        "#]],
    );
}

#[test]
fn test_if_else_stmts() {
    check_infer_with_code_flow_analysis(
        r#"
cond = 1 < 2
def f():
    x = 0
    if cond:
        x = "less"
    else:
        x = 1.
    x
"#,
        expect![[r#"
            1..5 "cond": bool
            8..9 "1": Literal[1]
            12..13 "2": Literal[2]
            8..13 "1 < 2": bool
            27..28 "x": Literal[0]
            31..32 "0": Literal[0]
            40..44 "cond": bool
            54..55 "x": Literal["less"]
            58..64 "\"less\"": Literal["less"]
            83..84 "x": float
            87..89 "1.": float
            94..95 "x": string | float
        "#]],
    );
}

#[test]
fn test_nested_if_stmts() {
    check_infer_with_code_flow_analysis(
        r#"
cond = 1 < 2
def f():
    x = 0
    if cond:
        x = "less"
    else:
        if cond:
            x = 1.
        elif cond:
            x = b""
        x
        if cond:
            x = True
        x
    x
"#,
        expect![[r#"
            1..5 "cond": bool
            8..9 "1": Literal[1]
            12..13 "2": Literal[2]
            8..13 "1 < 2": bool
            27..28 "x": Literal[0]
            31..32 "0": Literal[0]
            40..44 "cond": bool
            54..55 "x": Literal["less"]
            58..64 "\"less\"": Literal["less"]
            86..90 "cond": bool
            104..105 "x": float
            108..110 "1.": float
            124..128 "cond": bool
            142..143 "x": bytes
            146..149 "b\"\"": bytes
            158..159 "x": float | bytes | int
            171..175 "cond": bool
            189..190 "x": Literal[True]
            193..197 "True": Literal[True]
            206..207 "x": bool | float | bytes | int
            212..213 "x": string | bool | float | bytes | int
        "#]],
    );
}

#[test]
fn test_possibly_unbound() {
    check_infer_with_code_flow_analysis(
        r#"
def f():
    if 1 < 2:
        x = 1
    x
"#,
        expect![[r#"
            17..18 "1": Literal[1]
            21..22 "2": Literal[2]
            17..22 "1 < 2": bool
            32..33 "x": Literal[1]
            36..37 "1": Literal[1]
            42..43 "x": int | Unbound

            42..43 "x" is possibly unbound
        "#]],
    )
}

#[test]
fn test_unreachable() {
    check_infer_with_code_flow_analysis(
        r#"
def f():
    for x in 1, 2, 3:
        break
        y = 1
        y

def g():
    for x in 1, 2, 3:
        continue
        z = 2
        z

def h():
    for x in 1, 2, 3:
        if x < 1:
            y = 1
            break
        else:
            y = "one"
            break
        y
"#,
        expect![[r#"
            18..19 "x": int
            23..24 "1": Literal[1]
            26..27 "2": Literal[2]
            29..30 "3": Literal[3]
            23..30 "1, 2, 3": tuple[Literal[1], Literal[2], Literal[3]]
            54..55 "y": Never
            58..59 "1": Literal[1]
            68..69 "y": Never
            88..89 "x": int
            93..94 "1": Literal[1]
            96..97 "2": Literal[2]
            99..100 "3": Literal[3]
            93..100 "1, 2, 3": tuple[Literal[1], Literal[2], Literal[3]]
            127..128 "z": Never
            131..132 "2": Literal[2]
            141..142 "z": Never
            161..162 "x": int
            166..167 "1": Literal[1]
            169..170 "2": Literal[2]
            172..173 "3": Literal[3]
            166..173 "1, 2, 3": tuple[Literal[1], Literal[2], Literal[3]]
            186..187 "x": int
            190..191 "1": Literal[1]
            186..191 "x < 1": bool
            205..206 "y": Literal[1]
            209..210 "1": Literal[1]
            255..256 "y": Literal["one"]
            259..264 "\"one\"": Literal["one"]
            291..292 "y": Never

            54..69 Code is unreachable
            127..142 Code is unreachable
            291..292 Code is unreachable
        "#]],
    );
}

#[test]
fn test_for() {
    check_infer_with_code_flow_analysis(
        r#"
def f():
    x = 1
    for y in 1, 2, 3:
        x = "one"
    x
"#,
        expect![[r#"
            14..15 "x": Literal[1]
            18..19 "1": Literal[1]
            28..29 "y": int
            33..34 "1": Literal[1]
            36..37 "2": Literal[2]
            39..40 "3": Literal[3]
            33..40 "1, 2, 3": tuple[Literal[1], Literal[2], Literal[3]]
            50..51 "x": Literal["one"]
            54..59 "\"one\"": Literal["one"]
            64..65 "x": string | int
        "#]],
    );
}

#[test]
fn test_unused_definitions() {
    check_infer_with_unused_definitions(
        r#"
x = 1
x = 2
_y = []

def foo():
    x = 123
    y = "foo"
    y = y + "bar"

    def bar():
        pass

    def baz():
        pass

    baz()

def _foo():
    pass

def _bar():
    pass

_bar()

def baz():
    if True:
        x = 1
    else:
        x = 2
    print(x)
    _ = 123
    a, b = 3, 4
    (d, (e, f)) = 123
"#,
        expect![[r#"
            1..2 "x": Literal[1]
            5..6 "1": Literal[1]
            7..8 "x": Literal[2]
            11..12 "2": Literal[2]
            13..15 "_y": list[Unknown]
            18..20 "[]": list[Unknown]
            37..38 "x": Literal[123]
            41..44 "123": Literal[123]
            49..50 "y": Literal["foo"]
            53..58 "\"foo\"": Literal["foo"]
            63..64 "y": Literal["foobar"]
            67..68 "y": Literal["foo"]
            71..76 "\"bar\"": Literal["bar"]
            67..76 "y + \"bar\"": Literal["foobar"]
            140..143 "baz": def baz() -> Unknown
            140..145 "baz()": Unknown
            191..195 "_bar": def _bar() -> Unknown
            191..197 "_bar()": Unknown
            217..221 "True": Literal[True]
            231..232 "x": Literal[1]
            235..236 "1": Literal[1]
            255..256 "x": Literal[2]
            259..260 "2": Literal[2]
            265..270 "print": def print(*args: Any, str: string = None) -> None
            271..272 "x": Literal[2]
            265..273 "print(x)": None
            278..279 "_": Literal[123]
            282..285 "123": Literal[123]
            290..291 "a": Literal[3]
            293..294 "b": Literal[4]
            290..294 "a, b": tuple[Literal[3], Literal[4]]
            297..298 "3": Literal[3]
            300..301 "4": Literal[4]
            297..301 "3, 4": tuple[Literal[3], Literal[4]]
            307..308 "d": Unknown
            311..312 "e": Unknown
            314..315 "f": Unknown
            310..316 "(e, f)": Unknown
            306..317 "(d, (e, f))": tuple[Unknown, Unknown]
            320..323 "123": Literal[123]

            13..15 "_y" is not accessed
            37..38 "x" is not accessed
            63..64 "y" is not accessed
            86..89 "bar" is not accessed
            151..155 "_foo" is not accessed
            290..291 "a" is not accessed
            293..294 "b" is not accessed
            307..308 "d" is not accessed
            311..312 "e" is not accessed
            314..315 "f" is not accessed
            320..323 Type "Literal[123]" is not iterable
        "#]],
    );
}

#[test]
fn test_unreachable_fail() {
    check_infer_with_code_flow_analysis(
        r#"
def foo():
    if 1 < 2:
        x = "abc"
    elif 2 < 1:
        x = 0
    else:
        fail()
    x

x = 123
fail("hehe")
y = 234
"#,
        expect![[r#"
            19..20 "1": Literal[1]
            23..24 "2": Literal[2]
            19..24 "1 < 2": bool
            34..35 "x": Literal["abc"]
            38..43 "\"abc\"": Literal["abc"]
            53..54 "2": Literal[2]
            57..58 "1": Literal[1]
            53..58 "2 < 1": bool
            68..69 "x": Literal[0]
            72..73 "0": Literal[0]
            92..96 "fail": def fail(*args: Any) -> Never
            92..98 "fail()": Never
            103..104 "x": string | int
            106..107 "x": Literal[123]
            110..113 "123": Literal[123]
            114..118 "fail": def fail(*args: Any) -> Never
            119..125 "\"hehe\"": Literal["hehe"]
            114..126 "fail(\"hehe\")": Never
            127..128 "y": Literal[234]
            131..134 "234": Literal[234]

            127..134 Code is unreachable
        "#]],
    );
}

#[test]
fn test_definitions_beyond_current_scope() {
    check_infer_with_code_flow_analysis(
        r#"
foo = 123
[x + foo for x in [1, 2, 3]]
lambda x: x + foo
def f():
    foo
    y = 1
    maybe = 123
    def bar():
        foo + y
        maybe
    maybe = "abc"
"#,
        expect![[r#"
            1..4 "foo": Literal[123]
            7..10 "123": Literal[123]
            12..13 "x": int
            16..19 "foo": int
            12..19 "x + foo": int
            24..25 "x": int
            30..31 "1": Literal[1]
            33..34 "2": Literal[2]
            36..37 "3": Literal[3]
            29..38 "[1, 2, 3]": list[int]
            11..39 "[x + foo for x in [1, 2, 3]]": list[int]
            50..51 "x": Unknown
            54..57 "foo": int
            50..57 "x + foo": Unknown
            40..57 "lambda x: x + foo": def lambda(x) -> Unknown
            71..74 "foo": int
            79..80 "y": Literal[1]
            83..84 "1": Literal[1]
            89..94 "maybe": Literal[123]
            97..100 "123": Literal[123]
            124..127 "foo": int
            130..131 "y": int
            124..131 "foo + y": int
            140..145 "maybe": string | int
            150..155 "maybe": Literal["abc"]
            158..163 "\"abc\"": Literal["abc"]
        "#]],
    );
}
