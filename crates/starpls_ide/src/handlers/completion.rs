use crate::{
    completions::{self, CompletionItem},
    Database, FilePosition,
};

pub(crate) fn completion(
    db: &Database,
    pos: FilePosition,
    trigger_character: Option<String>,
) -> Option<Vec<CompletionItem>> {
    completions::completions(db, pos, trigger_character)
}
