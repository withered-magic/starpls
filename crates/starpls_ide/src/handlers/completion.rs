use crate::{
    completions::{self, CompletionItem},
    Database, FilePosition,
};

pub(crate) fn completion(db: &Database, pos: FilePosition) -> Option<Vec<CompletionItem>> {
    completions::completions(db, pos)
}
