use starpls_syntax::{SyntaxKind, SyntaxToken, TokenAtOffset};

pub(crate) fn pick_best_token(
    tokens: TokenAtOffset<SyntaxToken>,
    mut f: impl FnMut(SyntaxKind) -> usize,
) -> Option<SyntaxToken> {
    tokens.max_by_key(|token| f(token.kind()))
}
