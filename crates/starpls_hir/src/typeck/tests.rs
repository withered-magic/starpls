use crate::{source_map, test_database::TestDatabaseBuilder, Db as _, DisplayWithDb};
use expect_test::{expect, Expect};
use itertools::Itertools;
use starpls_bazel::APIContext;
use starpls_common::{parse, Db as _, Dialect, FileId};
use starpls_syntax::ast::AstNode;
use starpls_test_util::FixtureType;
use std::{cmp::Ordering, fmt::Write};

fn check_infer(input: &str, expect: Expect) {
    let mut builder = TestDatabaseBuilder::default();
    builder.add_function("provider");
    builder.add_function("struct");
    builder.add_type(FixtureType {
        name: "File".to_string(),
        fields: vec![],
    });
    builder.add_type(FixtureType {
        name: "ctx".to_string(),
        fields: vec![
            ("executable".to_string(), "struct".to_string()),
            ("file".to_string(), "struct".to_string()),
            ("files".to_string(), "struct".to_string()),
            ("outputs".to_string(), "struct".to_string()),
        ],
    });

    let mut db = builder.build();
    let file_id = FileId(0);
    let file = db.create_file(
        file_id,
        Dialect::Bazel,
        Some(APIContext::Bzl),
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
fn test_string_repetition() {
    check_infer(
        r#"
"abc" * 3
3 * "abc"
"#,
        expect![[r#"
            1..6 "\"abc\"": Literal["abc"]
            9..10 "3": Literal[3]
            1..10 "\"abc\" * 3": string
            11..12 "3": Literal[3]
            15..20 "\"abc\"": Literal["abc"]
            11..20 "3 * \"abc\"": string
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
    )
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
