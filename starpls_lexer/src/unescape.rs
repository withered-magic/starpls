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

pub fn unescape_string<F>(input: &str, callback: &mut F)
where
    F: FnMut(Range<usize>, Result<char, EscapeError>),
{
    let mut chars = input.chars();

    // Iterate over the input character by character, processing escape sequences as we see them.
    while let Some(c) = chars.next() {
        let start = input.len() - chars.as_str().len() - c.len_utf8();
        let res = match c {
            '\\' => match scan_escape(&mut chars) {
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

fn scan_escape(chars: &mut Chars<'_>) -> Result<Option<char>, EscapeError> {
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

        '0'..='7' => scan_octal_escape(chars)?,

        'x' => scan_hexadecimal_escape(chars)?,

        'u' | _ => return Err(EscapeError::InvalidEscape),
    };
    Ok(Some(c))
}

// pub fn unescape_literal<F>(input: &str, mode: Mode, callback: &mut F)
// where
//     F: FnMut(Range<usize>, Result<char, EscapeError>),
// {

// }

// #[derive(Debug, PartialEq, Eq)]
// pub enum Mode {
//     Str,
//     ByteStr,
//     RawStr,
//     RawByteStr,
// }
