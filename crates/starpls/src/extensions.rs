use lsp_types::request::Request;
use lsp_types::TextDocumentIdentifier;
use serde::Deserialize;
use serde::Serialize;

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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShowHirParams {
    pub text_document: TextDocumentIdentifier,
}

#[derive(Debug)]
pub enum ShowHir {}

impl Request for ShowHir {
    type Params = ShowHirParams;
    type Result = String;
    const METHOD: &'static str = "starpls/showHir";
}
