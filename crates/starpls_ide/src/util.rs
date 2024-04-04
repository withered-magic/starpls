use starpls_syntax::{SyntaxKind, SyntaxToken, TokenAtOffset};

pub(crate) fn pick_best_token(
    tokens: TokenAtOffset<SyntaxToken>,
    mut f: impl FnMut(SyntaxKind) -> usize,
) -> Option<SyntaxToken> {
    tokens.max_by_key(|token| f(token.kind()))
}

// TODO(withered-magic): This logic should probably be more sophisticated, but it works well
// enough for now.
pub(crate) fn unindent_doc(doc: &str) -> String {
    unindent::unindent(doc)
        .lines()
        .map(|line| {
            let trimmed = line.trim_start();
            let num_trimmed = line.len() - trimmed.len();
            let mut s = String::new();
            (0..num_trimmed).for_each(|_| s.push_str("&nbsp;"));
            s.push_str(trimmed);
            s.push_str("  ");
            s
        })
        .collect::<Vec<_>>()
        .join("\n")
}
