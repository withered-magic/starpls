use prost::Message;

use crate::{
    build::{attribute::Discriminator, AttributeDefinition, BuildLanguage, RuleDefinition},
    builtin::{Callable, Param, Value},
    Builtins,
};

pub fn decode_rules(build_language_output: &[u8]) -> anyhow::Result<Builtins> {
    let build_language = BuildLanguage::decode(&build_language_output[..])?;
    Ok(Builtins {
        global: build_language
            .rule
            .into_iter()
            .map(
                |RuleDefinition {
                     name,
                     documentation,
                     attribute,
                     ..
                 }| {
                    Value {
                        name,
                        doc: documentation.unwrap_or_else(|| String::new()),
                        callable: Some(Callable {
                            param: attribute
                                .into_iter()
                                .map(|AttributeDefinition { name, r#type, .. }| Param {
                                    name,
                                    r#type: attribute_type_string_from_i32(r#type),
                                    ..Default::default()
                                })
                                .collect(),
                            return_type: "None".to_string(),
                        }),
                        ..Default::default()
                    }
                },
            )
            .collect(),
        ..Default::default()
    })
}

pub fn attribute_type_string_from_i32(value: i32) -> String {
    use Discriminator::*;

    let discriminator = match Discriminator::try_from(value).ok() {
        Some(discriminator) => discriminator,
        None => return "".to_string(),
    };

    match discriminator {
        Integer => "int",
        String | License => "string",
        Label => "Label",
        StringList => "List of strings",
        LabelList => "List ofLabels",
        Boolean => "boolean",
        IntegerList => "List of ints",
        Output
        | OutputList
        | DistributionSet
        | StringDict
        | FilesetEntryList
        | LabelListDict
        | StringListDict
        | Tristate
        | Unknown
        | LabelDictUnary
        | SelectorList
        | LabelKeyedStringDict
        | DeprecatedStringDictUnary => "Unknown",
    }
    .to_string()
}
