use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CommonAttributes {
    pub build: Vec<Attribute>,
    pub repository: Vec<Attribute>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum AttributeKind {
    #[serde(rename = "boolean")]
    Bool,
    #[serde(rename = "int")]
    Int,
    #[serde(rename = "List of ints")]
    IntList,
    #[serde(rename = "Label")]
    Label,
    // TODO(withered-magic): Add a `rename`.
    LabelKeyedStringDict,
    #[serde(rename = "List of Labels")]
    LabelList,
    // TODO(withered-magic): Add a `rename`.
    Output,
    #[serde(rename = "List of Outputs")]
    OutputList,
    #[serde(rename = "string")]
    String,
    #[serde(rename = "List of strings")]
    StringList,
    #[serde(rename = "Dictionary of strings")]
    StringDict,
    // TODO(withered-magic): Add a `rename`.
    StringKeyedLabelDict,
    // TODO(withered-magic): Add a `rename`.
    StringListDict,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Attribute {
    pub name: String,
    pub r#type: AttributeKind,
    pub doc: String,
    pub default_value: String,
    pub is_mandatory: bool,
}

pub fn make_common_attributes() -> CommonAttributes {
    serde_json::from_str(include_str!("../data/commonAttributes.json"))
        .expect("bug: invalid commonAttributes.json")
}
