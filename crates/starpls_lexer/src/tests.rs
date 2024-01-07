use super::*;
use expect_test::{expect, Expect};

fn check_lexing(input: &str, expect: Expect) {
    let actual: String = tokenize(input)
        .map(|token| format!("{:?}\n", token))
        .collect();
    expect.assert_eq(&actual);
}

#[test]
fn smoke_test() {
    check_lexing(
        r#"
def _hello_world_impl(ctx):
    out = ctx.actions.declare_file(ctx.label.name + ".cc") # test comment
    ctx.actions.expand_template(
        output = out,
        template = ctx.file.template,
        substitutions = {"{NAME}": ctx.attr.username},
    )
    return [DefaultInfo(files = depset([out]))]

hello_world = rule(
    implementation = _hello_world_impl,
    attrs = {
        "username": attr.string(default = "unknown person"),
        "template": attr.label(
            allow_single_file = [".cc.tpl"],
            mandatory = True,
        ),
    },
)
    "#,
        expect![[r#"
            Token { kind: Newline, len: 1 }
            Token { kind: Def, len: 3 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Ident, len: 17 }
            Token { kind: OpenParen, len: 1 }
            Token { kind: Ident, len: 3 }
            Token { kind: CloseParen, len: 1 }
            Token { kind: Colon, len: 1 }
            Token { kind: Newline, len: 1 }
            Token { kind: Indent, len: 4 }
            Token { kind: Ident, len: 3 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Ident, len: 3 }
            Token { kind: Dot, len: 1 }
            Token { kind: Ident, len: 7 }
            Token { kind: Dot, len: 1 }
            Token { kind: Ident, len: 12 }
            Token { kind: OpenParen, len: 1 }
            Token { kind: Ident, len: 3 }
            Token { kind: Dot, len: 1 }
            Token { kind: Ident, len: 5 }
            Token { kind: Dot, len: 1 }
            Token { kind: Ident, len: 4 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Plus, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: Str { terminated: true, triple_quoted: false } }, len: 5 }
            Token { kind: CloseParen, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Comment, len: 14 }
            Token { kind: Newline, len: 1 }
            Token { kind: Whitespace, len: 4 }
            Token { kind: Ident, len: 3 }
            Token { kind: Dot, len: 1 }
            Token { kind: Ident, len: 7 }
            Token { kind: Dot, len: 1 }
            Token { kind: Ident, len: 15 }
            Token { kind: OpenParen, len: 1 }
            Token { kind: Whitespace, len: 9 }
            Token { kind: Ident, len: 6 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Ident, len: 3 }
            Token { kind: Comma, len: 1 }
            Token { kind: Whitespace, len: 9 }
            Token { kind: Ident, len: 8 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Ident, len: 3 }
            Token { kind: Dot, len: 1 }
            Token { kind: Ident, len: 4 }
            Token { kind: Dot, len: 1 }
            Token { kind: Ident, len: 8 }
            Token { kind: Comma, len: 1 }
            Token { kind: Whitespace, len: 9 }
            Token { kind: Ident, len: 13 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: OpenBrace, len: 1 }
            Token { kind: Literal { kind: Str { terminated: true, triple_quoted: false } }, len: 8 }
            Token { kind: Colon, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Ident, len: 3 }
            Token { kind: Dot, len: 1 }
            Token { kind: Ident, len: 4 }
            Token { kind: Dot, len: 1 }
            Token { kind: Ident, len: 8 }
            Token { kind: CloseBrace, len: 1 }
            Token { kind: Comma, len: 1 }
            Token { kind: Whitespace, len: 5 }
            Token { kind: CloseParen, len: 1 }
            Token { kind: Newline, len: 1 }
            Token { kind: Whitespace, len: 4 }
            Token { kind: Return, len: 6 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: OpenBrack, len: 1 }
            Token { kind: Ident, len: 11 }
            Token { kind: OpenParen, len: 1 }
            Token { kind: Ident, len: 5 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Ident, len: 6 }
            Token { kind: OpenParen, len: 1 }
            Token { kind: OpenBrack, len: 1 }
            Token { kind: Ident, len: 3 }
            Token { kind: CloseBrack, len: 1 }
            Token { kind: CloseParen, len: 1 }
            Token { kind: CloseParen, len: 1 }
            Token { kind: CloseBrack, len: 1 }
            Token { kind: Newline, len: 1 }
            Token { kind: Newline, len: 1 }
            Token { kind: Dedent { consistent: true }, len: 0 }
            Token { kind: Ident, len: 11 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Ident, len: 4 }
            Token { kind: OpenParen, len: 1 }
            Token { kind: Whitespace, len: 5 }
            Token { kind: Ident, len: 14 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Ident, len: 17 }
            Token { kind: Comma, len: 1 }
            Token { kind: Whitespace, len: 5 }
            Token { kind: Ident, len: 5 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: OpenBrace, len: 1 }
            Token { kind: Whitespace, len: 9 }
            Token { kind: Literal { kind: Str { terminated: true, triple_quoted: false } }, len: 10 }
            Token { kind: Colon, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Ident, len: 4 }
            Token { kind: Dot, len: 1 }
            Token { kind: Ident, len: 6 }
            Token { kind: OpenParen, len: 1 }
            Token { kind: Ident, len: 7 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: Str { terminated: true, triple_quoted: false } }, len: 16 }
            Token { kind: CloseParen, len: 1 }
            Token { kind: Comma, len: 1 }
            Token { kind: Whitespace, len: 9 }
            Token { kind: Literal { kind: Str { terminated: true, triple_quoted: false } }, len: 10 }
            Token { kind: Colon, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Ident, len: 4 }
            Token { kind: Dot, len: 1 }
            Token { kind: Ident, len: 5 }
            Token { kind: OpenParen, len: 1 }
            Token { kind: Whitespace, len: 13 }
            Token { kind: Ident, len: 17 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: OpenBrack, len: 1 }
            Token { kind: Literal { kind: Str { terminated: true, triple_quoted: false } }, len: 9 }
            Token { kind: CloseBrack, len: 1 }
            Token { kind: Comma, len: 1 }
            Token { kind: Whitespace, len: 13 }
            Token { kind: Ident, len: 9 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: True, len: 4 }
            Token { kind: Comma, len: 1 }
            Token { kind: Whitespace, len: 9 }
            Token { kind: CloseParen, len: 1 }
            Token { kind: Comma, len: 1 }
            Token { kind: Whitespace, len: 5 }
            Token { kind: CloseBrace, len: 1 }
            Token { kind: Comma, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: CloseParen, len: 1 }
            Token { kind: Newline, len: 1 }
            Token { kind: Whitespace, len: 4 }
        "#]],
    );
}

#[test]
fn test_decimal_numbers() {
    check_lexing(
        r#"
0
0.
0.e1
.0
0.0
.e1
1
1.
.1
.1e1
.1e+1
.1e-1
1e1
1e+1
1e-1
123
123e45
999999999999999999999999999999999999999999999999999
12345678901234567890
    "#,
        expect![[r#"
            Token { kind: Newline, len: 1 }
            Token { kind: Literal { kind: Int { base: Decimal, empty_int: false } }, len: 1 }
            Token { kind: Newline, len: 1 }
            Token { kind: Literal { kind: Float { empty_exponent: false } }, len: 2 }
            Token { kind: Newline, len: 1 }
            Token { kind: Literal { kind: Float { empty_exponent: false } }, len: 4 }
            Token { kind: Newline, len: 1 }
            Token { kind: Literal { kind: Float { empty_exponent: false } }, len: 2 }
            Token { kind: Newline, len: 1 }
            Token { kind: Literal { kind: Float { empty_exponent: false } }, len: 3 }
            Token { kind: Newline, len: 1 }
            Token { kind: Dot, len: 1 }
            Token { kind: Ident, len: 2 }
            Token { kind: Newline, len: 1 }
            Token { kind: Literal { kind: Int { base: Decimal, empty_int: false } }, len: 1 }
            Token { kind: Newline, len: 1 }
            Token { kind: Literal { kind: Float { empty_exponent: false } }, len: 2 }
            Token { kind: Newline, len: 1 }
            Token { kind: Literal { kind: Float { empty_exponent: false } }, len: 2 }
            Token { kind: Newline, len: 1 }
            Token { kind: Literal { kind: Float { empty_exponent: false } }, len: 4 }
            Token { kind: Newline, len: 1 }
            Token { kind: Literal { kind: Float { empty_exponent: false } }, len: 5 }
            Token { kind: Newline, len: 1 }
            Token { kind: Literal { kind: Float { empty_exponent: false } }, len: 5 }
            Token { kind: Newline, len: 1 }
            Token { kind: Literal { kind: Float { empty_exponent: false } }, len: 3 }
            Token { kind: Newline, len: 1 }
            Token { kind: Literal { kind: Float { empty_exponent: false } }, len: 4 }
            Token { kind: Newline, len: 1 }
            Token { kind: Literal { kind: Float { empty_exponent: false } }, len: 4 }
            Token { kind: Newline, len: 1 }
            Token { kind: Literal { kind: Int { base: Decimal, empty_int: false } }, len: 3 }
            Token { kind: Newline, len: 1 }
            Token { kind: Literal { kind: Float { empty_exponent: false } }, len: 6 }
            Token { kind: Newline, len: 1 }
            Token { kind: Literal { kind: Int { base: Decimal, empty_int: false } }, len: 51 }
            Token { kind: Newline, len: 1 }
            Token { kind: Literal { kind: Int { base: Decimal, empty_int: false } }, len: 20 }
            Token { kind: Newline, len: 1 }
            Token { kind: Whitespace, len: 4 }
        "#]],
    );
}

#[test]
fn test_hexadecimal_numbers() {
    check_lexing(
        r#"
0xA
0xAAG
0xG
0XA
0XG
0xA.
0xA.e1
0x12345678deadbeef12345678
    "#,
        expect![[r#"
            Token { kind: Newline, len: 1 }
            Token { kind: Literal { kind: Int { base: Hexadecimal, empty_int: false } }, len: 3 }
            Token { kind: Newline, len: 1 }
            Token { kind: Literal { kind: Int { base: Hexadecimal, empty_int: false } }, len: 4 }
            Token { kind: Ident, len: 1 }
            Token { kind: Newline, len: 1 }
            Token { kind: Literal { kind: Int { base: Hexadecimal, empty_int: true } }, len: 2 }
            Token { kind: Ident, len: 1 }
            Token { kind: Newline, len: 1 }
            Token { kind: Literal { kind: Int { base: Hexadecimal, empty_int: false } }, len: 3 }
            Token { kind: Newline, len: 1 }
            Token { kind: Literal { kind: Int { base: Hexadecimal, empty_int: true } }, len: 2 }
            Token { kind: Ident, len: 1 }
            Token { kind: Newline, len: 1 }
            Token { kind: Literal { kind: Int { base: Hexadecimal, empty_int: false } }, len: 3 }
            Token { kind: Dot, len: 1 }
            Token { kind: Newline, len: 1 }
            Token { kind: Literal { kind: Int { base: Hexadecimal, empty_int: false } }, len: 3 }
            Token { kind: Dot, len: 1 }
            Token { kind: Ident, len: 2 }
            Token { kind: Newline, len: 1 }
            Token { kind: Literal { kind: Int { base: Hexadecimal, empty_int: false } }, len: 26 }
            Token { kind: Newline, len: 1 }
            Token { kind: Whitespace, len: 4 }
        "#]],
    );
}

#[test]
fn test_octal_numbers() {
    check_lexing(
        r#"
0o123
0o12834
0o12934
0o12934.1
0o12934e1
0o123.
0o123.1
0123
012834
012934
"#,
        expect![[r#"
            Token { kind: Newline, len: 1 }
            Token { kind: Literal { kind: Int { base: Octal, empty_int: false } }, len: 5 }
            Token { kind: Newline, len: 1 }
            Token { kind: Literal { kind: Int { base: Octal, empty_int: false } }, len: 4 }
            Token { kind: Literal { kind: Int { base: Decimal, empty_int: false } }, len: 3 }
            Token { kind: Newline, len: 1 }
            Token { kind: Literal { kind: Int { base: Octal, empty_int: false } }, len: 4 }
            Token { kind: Literal { kind: Int { base: Decimal, empty_int: false } }, len: 3 }
            Token { kind: Newline, len: 1 }
            Token { kind: Literal { kind: Int { base: Octal, empty_int: false } }, len: 4 }
            Token { kind: Literal { kind: Float { empty_exponent: false } }, len: 5 }
            Token { kind: Newline, len: 1 }
            Token { kind: Literal { kind: Int { base: Octal, empty_int: false } }, len: 4 }
            Token { kind: Literal { kind: Float { empty_exponent: false } }, len: 5 }
            Token { kind: Newline, len: 1 }
            Token { kind: Literal { kind: Int { base: Octal, empty_int: false } }, len: 5 }
            Token { kind: Dot, len: 1 }
            Token { kind: Newline, len: 1 }
            Token { kind: Literal { kind: Int { base: Octal, empty_int: false } }, len: 5 }
            Token { kind: Literal { kind: Float { empty_exponent: false } }, len: 2 }
            Token { kind: Newline, len: 1 }
            Token { kind: Literal { kind: Int { base: Decimal, empty_int: false } }, len: 4 }
            Token { kind: Newline, len: 1 }
            Token { kind: Literal { kind: Int { base: Decimal, empty_int: false } }, len: 6 }
            Token { kind: Newline, len: 1 }
            Token { kind: Literal { kind: Int { base: Decimal, empty_int: false } }, len: 6 }
            Token { kind: Newline, len: 1 }
        "#]],
    );
}

#[test]
fn test_punctuation() {
    check_lexing(
        r#"+ - * / // % ** ~ & | ^ << >> . , = ; : ( ) [ ] { } < > >= <= == != += -= *= /= //= %= &= |= ^= <<= >>="#,
        expect![[r#"
            Token { kind: Plus, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Minus, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Star, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Slash, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: SlashSlash, len: 2 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Mod, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: StarStar, len: 2 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Tilde, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Ampersand, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Bar, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Caret, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: LtLt, len: 2 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: GtGt, len: 2 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Dot, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Comma, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Semi, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Colon, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: OpenParen, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: CloseParen, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: OpenBrack, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: CloseBrack, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: OpenBrace, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: CloseBrace, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Lt, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Gt, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Ge, len: 2 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Le, len: 2 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: EqEq, len: 2 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: BangEq, len: 2 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: PlusEq, len: 2 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: MinusEq, len: 2 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: StarEq, len: 2 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: SlashEq, len: 2 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: SlashSlashEq, len: 3 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: ModEq, len: 2 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: AmpersandEq, len: 2 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: BarEq, len: 2 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: CaretEq, len: 2 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: LtLtEq, len: 3 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: GtGtEq, len: 3 }
        "#]],
    );
}

#[test]
fn test_strings() {
    check_lexing(
        r#"
x = 'a\nb'
x = "a\nb"
x = r'a\nb'
x = r"a\nb"
x = 'a\\\nb'
x = "a\\\nb"
x = '\''
x = "\""
x = r'\''
x = r"\""
x = '''\''''
x = """\""""
x = r'''\''''
x = r"""\""""
x = ''''a'b'c'''
x = """"a"b"c"""
x = '''a\nb'''
x = """a\nb"""
x = '''a\rb'''
x = """a\rb"""
x = '''a\r\nb'''
x = """a\r\nb"""
x = '''a\n\rb'''
x = """a\n\rb"""
x = r'a\\\nb'
x = r"a\\\nb"
x = r'a\\\rb'
x = r"a\\\rb"
x = r'a\\\r\nb'
x = r"a\\\r\nb"
"#,
        expect![[r#"
            Token { kind: Newline, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: Str { terminated: true, triple_quoted: false } }, len: 6 }
            Token { kind: Newline, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: Str { terminated: true, triple_quoted: false } }, len: 6 }
            Token { kind: Newline, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: RawStr { terminated: true, triple_quoted: false } }, len: 7 }
            Token { kind: Newline, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: RawStr { terminated: true, triple_quoted: false } }, len: 7 }
            Token { kind: Newline, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: Str { terminated: true, triple_quoted: false } }, len: 8 }
            Token { kind: Newline, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: Str { terminated: true, triple_quoted: false } }, len: 8 }
            Token { kind: Newline, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: Str { terminated: true, triple_quoted: false } }, len: 4 }
            Token { kind: Newline, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: Str { terminated: true, triple_quoted: false } }, len: 4 }
            Token { kind: Newline, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: RawStr { terminated: true, triple_quoted: false } }, len: 5 }
            Token { kind: Newline, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: RawStr { terminated: true, triple_quoted: false } }, len: 5 }
            Token { kind: Newline, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: Str { terminated: true, triple_quoted: true } }, len: 8 }
            Token { kind: Newline, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: Str { terminated: true, triple_quoted: true } }, len: 8 }
            Token { kind: Newline, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: RawStr { terminated: true, triple_quoted: true } }, len: 9 }
            Token { kind: Newline, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: RawStr { terminated: true, triple_quoted: true } }, len: 9 }
            Token { kind: Newline, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: Str { terminated: true, triple_quoted: true } }, len: 12 }
            Token { kind: Newline, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: Str { terminated: true, triple_quoted: true } }, len: 12 }
            Token { kind: Newline, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: Str { terminated: true, triple_quoted: true } }, len: 10 }
            Token { kind: Newline, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: Str { terminated: true, triple_quoted: true } }, len: 10 }
            Token { kind: Newline, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: Str { terminated: true, triple_quoted: true } }, len: 10 }
            Token { kind: Newline, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: Str { terminated: true, triple_quoted: true } }, len: 10 }
            Token { kind: Newline, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: Str { terminated: true, triple_quoted: true } }, len: 12 }
            Token { kind: Newline, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: Str { terminated: true, triple_quoted: true } }, len: 12 }
            Token { kind: Newline, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: Str { terminated: true, triple_quoted: true } }, len: 12 }
            Token { kind: Newline, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: Str { terminated: true, triple_quoted: true } }, len: 12 }
            Token { kind: Newline, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: RawStr { terminated: true, triple_quoted: false } }, len: 9 }
            Token { kind: Newline, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: RawStr { terminated: true, triple_quoted: false } }, len: 9 }
            Token { kind: Newline, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: RawStr { terminated: true, triple_quoted: false } }, len: 9 }
            Token { kind: Newline, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: RawStr { terminated: true, triple_quoted: false } }, len: 9 }
            Token { kind: Newline, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: RawStr { terminated: true, triple_quoted: false } }, len: 11 }
            Token { kind: Newline, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: RawStr { terminated: true, triple_quoted: false } }, len: 11 }
            Token { kind: Newline, len: 1 }
        "#]],
    );
}

#[test]
fn test_implicit_line_joining() {
    check_lexing(
        r#"
x = (1,
     2)
x = [1,
     2]
x = {1,
     2}
x = (1,]
     2)
    "#,
        expect![[r#"
            Token { kind: Newline, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: OpenParen, len: 1 }
            Token { kind: Literal { kind: Int { base: Decimal, empty_int: false } }, len: 1 }
            Token { kind: Comma, len: 1 }
            Token { kind: Whitespace, len: 6 }
            Token { kind: Literal { kind: Int { base: Decimal, empty_int: false } }, len: 1 }
            Token { kind: CloseParen, len: 1 }
            Token { kind: Newline, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: OpenBrack, len: 1 }
            Token { kind: Literal { kind: Int { base: Decimal, empty_int: false } }, len: 1 }
            Token { kind: Comma, len: 1 }
            Token { kind: Whitespace, len: 6 }
            Token { kind: Literal { kind: Int { base: Decimal, empty_int: false } }, len: 1 }
            Token { kind: CloseBrack, len: 1 }
            Token { kind: Newline, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: OpenBrace, len: 1 }
            Token { kind: Literal { kind: Int { base: Decimal, empty_int: false } }, len: 1 }
            Token { kind: Comma, len: 1 }
            Token { kind: Whitespace, len: 6 }
            Token { kind: Literal { kind: Int { base: Decimal, empty_int: false } }, len: 1 }
            Token { kind: CloseBrace, len: 1 }
            Token { kind: Newline, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: OpenParen, len: 1 }
            Token { kind: Literal { kind: Int { base: Decimal, empty_int: false } }, len: 1 }
            Token { kind: Comma, len: 1 }
            Token { kind: CloseBrack, len: 1 }
            Token { kind: Whitespace, len: 6 }
            Token { kind: Literal { kind: Int { base: Decimal, empty_int: false } }, len: 1 }
            Token { kind: CloseParen, len: 1 }
            Token { kind: Newline, len: 1 }
            Token { kind: Whitespace, len: 4 }
        "#]],
    );
}

#[test]
fn test_indentation() {
    check_lexing(
        r#"
def f():
    pass
    "#,
        expect![[r#"
            Token { kind: Newline, len: 1 }
            Token { kind: Def, len: 3 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: OpenParen, len: 1 }
            Token { kind: CloseParen, len: 1 }
            Token { kind: Colon, len: 1 }
            Token { kind: Newline, len: 1 }
            Token { kind: Indent, len: 4 }
            Token { kind: Pass, len: 4 }
            Token { kind: Newline, len: 1 }
            Token { kind: Whitespace, len: 4 }
            Token { kind: Dedent { consistent: true }, len: 0 }
        "#]],
    );
}
