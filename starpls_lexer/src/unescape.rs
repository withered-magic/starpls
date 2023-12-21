use std::ops::Range;
use std::str::Chars;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq, Eq)]
pub enum EscapeError {
    /// Escaped `\` character without continuation.
    LoneSlash,
    /// Invalid escape character (e.g., `\z`).
    InvalidEscape,

    // Octal escape-related errors.
    /// Value of octal escape is greater than 127 for normal strings, or 255 for byte strings.
    InvalidOctalEscape,

    // Hexadecimal escape-related errors.
    /// Empty escape sequence, e.g. `\x`.
    EmptyHexadecimalEscape,
    /// Less than 2 characters in hexadecimal escape sequence, e.g. `\xF`.
    TooShortHexadecimalEscape,
    /// Value of hexadecimal escape is greater than 127 for normal strings, or 255 for byte strings.
    InvalidHexadecimalEscape,

    // Unicode escape-related errors.
    /// Empty escape sequence, e.g. `\u`.
    EmptyUnicodeEscape,
    /// Less than 4 characters in Unicode escape sequence, e.g. `\uFF`.
    TooShort16BitUnicodeEscape,
    /// Greater than 4 but less than 8 characters in Unicode escape sequence, e.g. `\u10FFFF`.
    TooShort32BitUnicodeEscape,
    /// Greater than 8 characters in Unicode escape sequence, e.g. `\u0000010FFFF`.
    TooLong32BitUnicodeEscape,
}

/// Represents the type of string or bytes literal being unescaped. This is needed because strings and bytes literals,
/// as well as their raw counterparts, support different types of escapes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Mode {
    Str,
    RawStr,
    ByteStr,
    RawByteStr,
}

pub fn unescape_string<F>(input: &str, raw: bool, callback: &mut F)
where
    F: FnMut(Range<usize>, Result<char, EscapeError>),
{
    let mut chars = input.chars();

    // Iterate over the input character by character, processing escape sequences as we see them.
    while let Some(c) = chars.next() {
        let start = input.len() - chars.as_str().len() - c.len_utf8();
        let res = match c {
            '\\' => match scan_string_escape(&mut chars) {
                Ok(Some(c)) => Ok(c),
                Ok(None) => continue,
                Err(err) => Err(err),
            },
            _ => Ok(c),
        };
        let end = input.len() - chars.as_str().len();
        callback(start..end, res);
    }
}

fn scan_string_escape(chars: &mut Chars<'_>) -> Result<Option<char>, EscapeError> {
    let c = match chars.next().ok_or(EscapeError::LoneSlash)? {
        // Traditional escape sequences.
        // Taken from https://github.com/bazelbuild/starlark/blob/master/spec.md#string-escapes.
        'a' => '\x07', // alert or bell
        'b' => '\x08', // backspace
        'f' => '\x0C', // form feed
        'n' => '\x0A', // line feed
        'r' => '\x0D', // carriage return
        't' => '\x09', // horizontal tab
        'v' => '\x0B', // vertical tab

        '\\' => '\\',            // literal backslash
        '\n' => return Ok(None), // escaped newlines are ignored

        // octal escapes immediately begin with digits
        c @ '0'..='7' => {
            scan_string_octal_escape(chars, c.to_digit(8).expect("invalid octal digit"))?
        }

        // hexadecimal escapes start with `x`
        'x' => scan_string_hexadecimal_escape(chars)?, // hexadecimal escapes start with `x`

        // 'u' | 'U' => scan_unicode_escape(chars)?, // unicode escapes start with either `u` or `U`
        _ => return Err(EscapeError::InvalidEscape),
    };
    Ok(Some(c))
}

fn scan_string_octal_escape(
    chars: &mut Chars<'_>,
    initial_value: u32,
) -> Result<char, EscapeError> {
    let value = scan_octal_escape(chars, initial_value);
    if value > 127 {
        Err(EscapeError::InvalidOctalEscape)
    } else {
        Ok(char::from_u32(value).expect("invalid char value"))
    }
}

fn scan_byte_string_octal_escape(
    chars: &mut Chars<'_>,
    initial_value: u32,
) -> Result<u8, EscapeError> {
    let value = scan_octal_escape(chars, initial_value);
    if value > 255 {
        Err(EscapeError::InvalidOctalEscape)
    } else {
        Ok(u8::try_from(value).expect("value does not fit in u8"))
    }
}

/// Processes an octal escape, which consists of up to 3 octal digits.
fn scan_octal_escape(chars: &mut Chars<'_>, mut initial_value: u32) -> u32 {
    // Process up to 2 more octal digits.
    for _ in 0..2 {
        match chars.next() {
            Some(c @ '0'..='7') => {
                let digit = c.to_digit(8).expect("invalid octal digit");
                initial_value = initial_value * 8 + digit;
            }
            _ => break,
        }
    }
    initial_value
}

fn scan_string_hexadecimal_escape(chars: &mut Chars<'_>) -> Result<char, EscapeError> {
    let value = scan_hexadecimal_escape(chars)?;
    if value > 127 {
        Err(EscapeError::InvalidHexadecimalEscape)
    } else {
        // Safety: `value` is guaranteed to be valid ASCII.
        unsafe { Ok(char::from_u32_unchecked(value)) }
    }
}

fn scan_byte_string_hexadecimal_escape(chars: &mut Chars<'_>) -> Result<u8, EscapeError> {
    let value = scan_hexadecimal_escape(chars)?;
    if value > 255 {
        Err(EscapeError::InvalidHexadecimalEscape)
    } else {
        Ok(u8::try_from(value).expect("value does not fit in u8"))
    }
}

fn scan_hexadecimal_escape(chars: &mut Chars<'_>) -> Result<u32, EscapeError> {
    // Process up to 6 hexadecimal digits.
    let mut num_digits = 0;
    let mut value = 0;

    while let Some(c @ ('0'..='9' | 'a'..='f' | 'A'..='F')) = chars.next() {
        let digit = c.to_digit(16).expect("invalid hexadecimal digit");
        num_digits += 1;
        value = value * 16 + digit;
        if num_digits > 6 {
            break;
        }
    }

    if num_digits == 0 {
        Err(EscapeError::EmptyHexadecimalEscape)
    } else {
        Ok(value)
    }
}

fn scan_unicode_escape(chars: &mut Chars<'_>, mode: Mode) -> Result<char, EscapeError> {
    todo!()
}
