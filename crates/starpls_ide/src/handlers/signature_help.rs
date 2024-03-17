use crate::{
    signature_help::{self, SignatureHelp},
    Database, FilePosition,
};

pub(crate) fn signature_help(db: &Database, pos: FilePosition) -> Option<SignatureHelp> {
    signature_help::signature_help(db, pos)
}
