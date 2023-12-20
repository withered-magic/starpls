use super::*;
use expect_test::{expect, Expect};

fn check_lexing(input: &str, expect: Expect) {
    let actual: String = tokenize(input)
        .map(|token| format!("{:?}\n", token))
        .collect();
    expect.assert_eq(&actual);
}

// #[test]
// fn smoke_test() {
//     check_lexing(
//         r#"

//     "#,
//         expect![],
//     );
// }

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
            Token { kind: Unknown, len: 1 }
            Token { kind: Literal { kind: Int { base: Decimal, empty_int: false } }, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Literal { kind: Float { empty_exponent: false } }, len: 2 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Literal { kind: Float { empty_exponent: false } }, len: 4 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Literal { kind: Float { empty_exponent: false } }, len: 2 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Literal { kind: Float { empty_exponent: false } }, len: 3 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Dot, len: 1 }
            Token { kind: Ident, len: 2 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Literal { kind: Int { base: Decimal, empty_int: false } }, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Literal { kind: Float { empty_exponent: false } }, len: 2 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Literal { kind: Float { empty_exponent: false } }, len: 2 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Literal { kind: Float { empty_exponent: false } }, len: 4 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Literal { kind: Float { empty_exponent: false } }, len: 5 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Literal { kind: Float { empty_exponent: false } }, len: 5 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Literal { kind: Float { empty_exponent: false } }, len: 3 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Literal { kind: Float { empty_exponent: false } }, len: 4 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Literal { kind: Float { empty_exponent: false } }, len: 4 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Literal { kind: Int { base: Decimal, empty_int: false } }, len: 3 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Literal { kind: Float { empty_exponent: false } }, len: 6 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Literal { kind: Int { base: Decimal, empty_int: false } }, len: 51 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Literal { kind: Int { base: Decimal, empty_int: false } }, len: 20 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Unknown, len: 1 }
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
            Token { kind: Unknown, len: 1 }
            Token { kind: Literal { kind: Int { base: Hexadecimal, empty_int: false } }, len: 3 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Literal { kind: Int { base: Hexadecimal, empty_int: false } }, len: 4 }
            Token { kind: Ident, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Literal { kind: Int { base: Hexadecimal, empty_int: true } }, len: 2 }
            Token { kind: Ident, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Literal { kind: Int { base: Hexadecimal, empty_int: false } }, len: 3 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Literal { kind: Int { base: Hexadecimal, empty_int: true } }, len: 2 }
            Token { kind: Ident, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Literal { kind: Int { base: Hexadecimal, empty_int: false } }, len: 3 }
            Token { kind: Dot, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Literal { kind: Int { base: Hexadecimal, empty_int: false } }, len: 3 }
            Token { kind: Dot, len: 1 }
            Token { kind: Ident, len: 2 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Literal { kind: Int { base: Hexadecimal, empty_int: false } }, len: 26 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Unknown, len: 1 }
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
            Token { kind: Unknown, len: 1 }
            Token { kind: Literal { kind: Int { base: Octal, empty_int: false } }, len: 5 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Literal { kind: Int { base: Octal, empty_int: false } }, len: 4 }
            Token { kind: Literal { kind: Int { base: Decimal, empty_int: false } }, len: 3 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Literal { kind: Int { base: Octal, empty_int: false } }, len: 4 }
            Token { kind: Literal { kind: Int { base: Decimal, empty_int: false } }, len: 3 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Literal { kind: Int { base: Octal, empty_int: false } }, len: 4 }
            Token { kind: Literal { kind: Float { empty_exponent: false } }, len: 5 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Literal { kind: Int { base: Octal, empty_int: false } }, len: 4 }
            Token { kind: Literal { kind: Float { empty_exponent: false } }, len: 5 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Literal { kind: Int { base: Octal, empty_int: false } }, len: 5 }
            Token { kind: Dot, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Literal { kind: Int { base: Octal, empty_int: false } }, len: 5 }
            Token { kind: Literal { kind: Float { empty_exponent: false } }, len: 2 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Literal { kind: Int { base: Decimal, empty_int: false } }, len: 4 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Literal { kind: Int { base: Decimal, empty_int: false } }, len: 6 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Literal { kind: Int { base: Decimal, empty_int: false } }, len: 6 }
            Token { kind: Unknown, len: 1 }
        "#]],
    );
}

#[test]
fn test_punctuation() {
    check_lexing(
        r#"+ - * / // % ** ~ & | ^ << >> . , = ; : ( ) [ ] { } < > >= <= == != += -= *= /= //= %= &= |= ^= <<= >>="#,
        expect![[r#"
            Token { kind: Plus, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Minus, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Star, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Slash, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: SlashSlash, len: 2 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Mod, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: StarStar, len: 2 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Tilde, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Ampersand, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Bar, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Caret, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: LtLt, len: 2 }
            Token { kind: Unknown, len: 1 }
            Token { kind: GtGt, len: 2 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Dot, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Comma, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Semi, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Colon, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: OpenParen, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: CloseParen, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: OpenBrack, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: CloseBrack, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: OpenBrace, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: CloseBrace, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Lt, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Gt, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Ge, len: 2 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Le, len: 2 }
            Token { kind: Unknown, len: 1 }
            Token { kind: EqEq, len: 2 }
            Token { kind: Unknown, len: 1 }
            Token { kind: BangEq, len: 2 }
            Token { kind: Unknown, len: 1 }
            Token { kind: PlusEq, len: 2 }
            Token { kind: Unknown, len: 1 }
            Token { kind: MinusEq, len: 2 }
            Token { kind: Unknown, len: 1 }
            Token { kind: StarEq, len: 2 }
            Token { kind: Unknown, len: 1 }
            Token { kind: SlashEq, len: 2 }
            Token { kind: Unknown, len: 1 }
            Token { kind: SlashSlashEq, len: 3 }
            Token { kind: Unknown, len: 1 }
            Token { kind: ModEq, len: 2 }
            Token { kind: Unknown, len: 1 }
            Token { kind: AmpersandEq, len: 2 }
            Token { kind: Unknown, len: 1 }
            Token { kind: BarEq, len: 2 }
            Token { kind: Unknown, len: 1 }
            Token { kind: CaretEq, len: 2 }
            Token { kind: Unknown, len: 1 }
            Token { kind: LtLtEq, len: 3 }
            Token { kind: Unknown, len: 1 }
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
            Token { kind: Unknown, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Literal { kind: Str { terminated: true } }, len: 6 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Literal { kind: Str { terminated: true } }, len: 6 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Literal { kind: RawStr { terminated: true } }, len: 7 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Literal { kind: RawStr { terminated: true } }, len: 7 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Literal { kind: Str { terminated: true } }, len: 8 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Literal { kind: Str { terminated: true } }, len: 8 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Literal { kind: Str { terminated: true } }, len: 4 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Literal { kind: Str { terminated: true } }, len: 4 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Literal { kind: RawStr { terminated: true } }, len: 5 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Literal { kind: RawStr { terminated: true } }, len: 5 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Literal { kind: Str { terminated: true } }, len: 8 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Literal { kind: Str { terminated: true } }, len: 8 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Literal { kind: RawStr { terminated: true } }, len: 9 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Literal { kind: RawStr { terminated: true } }, len: 9 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Literal { kind: Str { terminated: true } }, len: 12 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Literal { kind: Str { terminated: true } }, len: 12 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Literal { kind: Str { terminated: true } }, len: 10 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Literal { kind: Str { terminated: true } }, len: 10 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Literal { kind: Str { terminated: true } }, len: 10 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Literal { kind: Str { terminated: true } }, len: 10 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Literal { kind: Str { terminated: true } }, len: 12 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Literal { kind: Str { terminated: true } }, len: 12 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Literal { kind: Str { terminated: true } }, len: 12 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Literal { kind: Str { terminated: true } }, len: 12 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Literal { kind: RawStr { terminated: true } }, len: 9 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Literal { kind: RawStr { terminated: true } }, len: 9 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Literal { kind: RawStr { terminated: true } }, len: 9 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Literal { kind: RawStr { terminated: true } }, len: 9 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Literal { kind: RawStr { terminated: true } }, len: 11 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Unknown, len: 1 }
            Token { kind: Literal { kind: RawStr { terminated: true } }, len: 11 }
            Token { kind: Unknown, len: 1 }
        "#]],
    );
}
