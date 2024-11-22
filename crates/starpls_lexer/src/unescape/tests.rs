use std::fmt::Write;
use std::str;

use expect_test::expect;
use expect_test::Expect;

use super::*;

#[test]
fn test_unescape_string() {}

#[test]
fn test_unescape_byte_string() {
    fn check(input: &str, expect: Expect) {
        let mut actual = String::new();
        unescape_byte_string(input, &mut |range, res| {
            write!(&mut actual, "{:?} ", range).unwrap();
            match res {
                Ok(b) => {
                    if let Ok(s) = str::from_utf8(b) {
                        writeln!(&mut actual, "{:?}", s).unwrap();
                    } else {
                        writeln!(&mut actual, "{:x?}", b).unwrap();
                    }
                }
                Err(err) => {
                    writeln!(&mut actual, "{:?}", err).unwrap();
                }
            }
        });
        expect.assert_eq(&actual);
    }

    check(
        "\\0",
        expect![[r#"
            0..2 "\0"
        "#]],
    );
    check(
        r#"AЀ世😿"#,
        expect![[r#"
            0..1 "A"
            1..3 "Ѐ"
            3..6 "世"
            6..10 "😿"
        "#]],
    );
    check(
        r#"\x41\u0400\u4e16\U0001F63F"#,
        expect![[r#"
        0..4 "A"
        4..10 "Ѐ"
        10..16 "世"
        16..26 "😿"
    "#]],
    );
    check(
        r#"\377\378\x80\xff\xff"#,
        expect![[r#"
        0..4 [ff]
        4..7 "\u{1f}"
        7..8 "8"
        8..12 [80]
        12..16 [ff]
        16..20 [ff]
    "#]],
    );
}
