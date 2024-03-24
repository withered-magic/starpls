use prost::Message;
use starpls_bazel::{
    build::{attribute::Discriminator, BuildLanguage},
    build_language::decode_rules,
    client::BazelCLI,
    load_builtins,
};
use std::collections::HashSet;

#[repr(u16)]
#[derive(Debug)]
enum CompletionItemRelevance {
    First,
    Second,
}

fn main() -> anyhow::Result<()> {
    let bazel_client = BazelCLI::new("bazel");
    let build_language_output = bazel_client.build_language()?;
    let BuildLanguage { rule: rules } = BuildLanguage::decode(&build_language_output[..])?;

    for rule in rules {
        eprintln!("rule: {:?}", rule.name);
        for attr in rule.attribute.into_iter().filter(|attr| {
            !attr.name.starts_with(&['$', ':'])
                && attr.r#type() == Discriminator::LabelKeyedStringDict
        }) {
            eprintln!(
                "attr: {}, {:?}, {:?}",
                attr.name,
                attr.r#type(),
                attr.documentation(),
            );
        }

        // for attr in rule.attribute.into_iter().take(1) {
        //     eprintln!("attr: {:?}", attr);
        // }
        // eprintln!("attribute: {:?}", rule.a)
    }

    Ok(())

    // let build_language = decode_rules(build_language_output)

    // let types: HashSet<_> = builtins
    //     .r#type
    //     .iter()
    //     .map(|type_| type_.name.clone())
    //     .collect();

    // println!("{:?}", builtins.global.len());

    // for builtin in builtins.global {
    //     println!("{:?} {:?} ", builtin.name, builtin.r#type,);
    // }

    // for builtin in builtins.r#type.iter() {
    //     println!("{:?}", builtin.name)
    // }

    // for global in builtins.global {
    //     if global.name == "Label" {
    //         eprintln!("{:?}", global.name);
    //         eprintln!("{:?}", global)
    //     }
    // }

    // for type_ in builtins.r#type.iter() {
    //     // if type_.name == "native" {
    //     //     for field in type_.field.iter() {
    //     //         eprintln!("{:?}", field.name);
    //     //     }
    //     // }
    // }

    // let mut types = HashSet::new();

    // for builtin in builtins.r#type.iter() {
    //     for field in builtin.field.iter() {
    //         if let Some(ref callable) = field.callable {
    //             for param in callable.param.iter() {
    //                 let mut s = String::new();
    //                 let mut chars = param.r#type.chars();
    //                 let mut in_tag = false;
    //                 while let Some(c) = chars.next() {
    //                     match (c, in_tag) {
    //                         ('<', false) => in_tag = true,
    //                         ('>', true) => in_tag = false,
    //                         (c, false) => s.push(c),
    //                         _ => {}
    //                     }
    //                 }

    //                 for el in s.split("; or ") {
    //                     if el.contains(" of ") {
    //                         // eprintln!("{}", el);
    //                     }
    //                     let mut parts = el.split(" of ");
    //                     if let Some(part) = parts.next() {
    //                         if !part.is_empty() {
    //                             types.insert(part.to_string());
    //                         }
    //                     }
    //                     if let Some(part) = parts.next() {
    //                         if !part.is_empty() {
    //                             types.insert(
    //                                 if part.ends_with('s') {
    //                                     &part[..part.len() - 1]
    //                                 } else {
    //                                     part
    //                                 }
    //                                 .to_string(),
    //                             );
    //                         }
    //                     }
    //                     // el.split(" of ")
    //                 }
    //             }

    //             // eprintln!("ret type {:?}", callable.return_type);

    //             let mut s = String::new();
    //             let mut chars = callable.return_type.chars();
    //             let mut in_tag = false;
    //             while let Some(c) = chars.next() {
    //                 match (c, in_tag) {
    //                     ('<', false) => in_tag = true,
    //                     ('>', true) => in_tag = false,
    //                     (c, false) => s.push(c),
    //                     _ => {}
    //                 }
    //             }

    //             // eprintln!("{:?}", s);

    //             for el in s.split("; or ") {
    //                 if el.contains(" of ") {
    //                     // eprintln!("{}", el);
    //                 }
    //                 let mut parts = el.split(" of ");
    //                 if let Some(part) = parts.next() {
    //                     if !part.is_empty() {
    //                         types.insert(part.to_string());
    //                     }
    //                 }
    //                 if let Some(part) = parts.next() {
    //                     if !part.is_empty() {
    //                         types.insert(
    //                             if part.ends_with('s') {
    //                                 &part[..part.len() - 1]
    //                             } else {
    //                                 part
    //                             }
    //                             .to_string(),
    //                         );
    //                     }
    //                 }
    //                 // el.split(" of ")
    //             }
    //             // for param in callable.param.iter() {
    //             //     let mut s = String::new();
    //             //     let mut chars = param.r#type.chars();
    //             //     let mut in_tag = true;
    //             //     while let Some(c) = chars.next() {
    //             //         match (c, in_tag) {

    //             //         }
    //             //     }
    //             //     let s: Vec<String> = s
    //             //         .replace("sequence", "list")
    //             //         .replace("Iterable", "list")
    //             //         .split("; or ")
    //             //         .map(|s| s.to_string())
    //             //         .collect();
    //             //     println!("{} {:?}", param.name, s);
    //             // }
    //             // eprintln!("{:?}", callable.return_type);
    //             // callable.
    //         }
    //         // if field.callable.is_none() {
    //         //     eprintln!("{:?}", field.r#type);
    //         // }
    //     }
    // }

    // for type_ in types {
    //     eprintln!("{}", type_)
    // }

    // Ok(())
}
