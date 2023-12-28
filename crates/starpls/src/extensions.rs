use lsp_types::{request::Request, TextDocumentIdentifier};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShowSyntaxTreeParams {
    pub text_document: TextDocumentIdentifier,
}

#[derive(Debug)]
pub enum ShowSyntaxTree {}

impl Request for ShowSyntaxTree {
    type Params = ShowSyntaxTreeParams;
    type Result = String;
    const METHOD: &'static str = "starpls/showSyntaxTree";
}
