use crate::{
    completion::{self, CompletionItem},
    Database, FilePosition,
};

pub(crate) fn completion(
    db: &Database,
    pos: FilePosition,
    trigger_character: Option<String>,
) -> Option<Vec<CompletionItem>> {
    completion::completions(db, pos, trigger_character)
}
