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
    let mut is_in_code_block = false;
    unindent::unindent(doc)
        .lines()
        .map(|line| {
            let trimmed = line.trim_start();
            let num_trimmed = line.len() - trimmed.len();
            let mut s = String::new();

            if trimmed.starts_with("```") {
                is_in_code_block = !is_in_code_block;
            }

            (0..num_trimmed)
                .for_each(|_| s.push_str(if is_in_code_block { " " } else { "&nbsp;" }));
            s.push_str(trimmed);
            s.push_str("  ");
            s
        })
        .collect::<Vec<_>>()
        .join("\n")
}
