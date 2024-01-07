use starpls_syntax::TextSize;

pub const CURSOR_MARKER: &str = "$0";

pub fn parse_fixture(fixture: &str) -> (String, TextSize) {
    let offset = fixture.find(CURSOR_MARKER).unwrap();
    let mut text = String::new();
    text.push_str(&fixture[..offset]);
    text.push_str(&fixture[offset + CURSOR_MARKER.len()..]);
    (text, (offset as u32).into())
}
