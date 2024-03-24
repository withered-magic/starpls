use prost::Message;

use crate::{
    build::{attribute::Discriminator, BuildLanguage, RuleDefinition},
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
                                .filter(|attr| !attr.name.starts_with(&['$', ':']))
                                .map(|attr| {
                                    let doc = attr.documentation().to_string();
                                    let r#type =
                                        attribute_type_string_from_discriminator(attr.r#type());
                                    Param {
                                        name: attr.name,
                                        r#type,
                                        doc,
                                        is_mandatory: false,
                                        ..Default::default()
                                    }
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

pub fn attribute_type_string_from_discriminator(value: Discriminator) -> String {
    use Discriminator::*;

    match value {
        Integer | Tristate => "int",
        String | License => "string",
        Label => "Label",
        StringList | DistributionSet => "List of strings",
        LabelList => "List of Labels",
        Boolean => "boolean",
        IntegerList => "List of ints",
        LabelListDict => "Dict of Labels",
        StringDict => "Dict of strings",
        // TODO(withered-magic): Handle StringListDict.
        StringListDict => "Unknown",
        // TODO(withered-magic): Handle LabelKeyedStringDict.
        LabelKeyedStringDict => "Unknown",
        _ => "Unknown",
    }
    .to_string()
}
