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
