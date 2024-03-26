use starpls_syntax::{SyntaxKind, SyntaxToken, TokenAtOffset};

pub(crate) fn pick_best_token(
    tokens: TokenAtOffset<SyntaxToken>,
    mut f: impl FnMut(SyntaxKind) -> usize,
) -> Option<SyntaxToken> {
    tokens.max_by_key(|token| f(token.kind()))
}

// TODO(withered-magic): This logic should probably be more sophisticated, but it works well
// enough for now.
pub(crate) fn deindent_doc(doc: &str) -> String {
    doc.lines()
        .map(|line| format!("{}  ", line.trim_start()))
        .collect::<Vec<_>>()
        .join("\n")
}
