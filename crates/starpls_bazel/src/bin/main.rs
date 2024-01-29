use starpls_bazel::load_builtins;
use std::collections::HashSet;

fn main() -> anyhow::Result<()> {
    let builtins = load_builtins("../../editors/code/builtin.pb")?;
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

    let mut types = HashSet::new();

    for builtin in builtins.r#type.iter() {
        for field in builtin.field.iter() {
            if let Some(ref callable) = field.callable {
                for param in callable.param.iter() {
                    let mut s = String::new();
                    let mut chars = param.r#type.chars();
                    let mut in_tag = false;
                    while let Some(c) = chars.next() {
                        match (c, in_tag) {
                            ('<', false) => in_tag = true,
                            ('>', true) => in_tag = false,
                            (c, false) => s.push(c),
                            _ => {}
                        }
                    }

                    for el in s.split("; or ") {
                        if el.contains(" of ") {
                            eprintln!("{}", el);
                        }
                        let mut parts = el.split(" of ");
                        if let Some(part) = parts.next() {
                            if !part.is_empty() {
                                types.insert(part.to_string());
                            }
                        }
                        if let Some(part) = parts.next() {
                            if !part.is_empty() {
                                types.insert(
                                    if part.ends_with('s') {
                                        &part[..part.len() - 1]
                                    } else {
                                        part
                                    }
                                    .to_string(),
                                );
                            }
                        }
                        // el.split(" of ")
                    }
                }

                // eprintln!("ret type {:?}", callable.return_type);

                let mut s = String::new();
                let mut chars = callable.return_type.chars();
                let mut in_tag = false;
                while let Some(c) = chars.next() {
                    match (c, in_tag) {
                        ('<', false) => in_tag = true,
                        ('>', true) => in_tag = false,
                        (c, false) => s.push(c),
                        _ => {}
                    }
                }

                // eprintln!("{:?}", s);

                for el in s.split("; or ") {
                    if el.contains(" of ") {
                        eprintln!("{}", el);
                    }
                    let mut parts = el.split(" of ");
                    if let Some(part) = parts.next() {
                        if !part.is_empty() {
                            types.insert(part.to_string());
                        }
                    }
                    if let Some(part) = parts.next() {
                        if !part.is_empty() {
                            types.insert(
                                if part.ends_with('s') {
                                    &part[..part.len() - 1]
                                } else {
                                    part
                                }
                                .to_string(),
                            );
                        }
                    }
                    // el.split(" of ")
                }
                // for param in callable.param.iter() {
                //     let mut s = String::new();
                //     let mut chars = param.r#type.chars();
                //     let mut in_tag = true;
                //     while let Some(c) = chars.next() {
                //         match (c, in_tag) {

                //         }
                //     }
                //     let s: Vec<String> = s
                //         .replace("sequence", "list")
                //         .replace("Iterable", "list")
                //         .split("; or ")
                //         .map(|s| s.to_string())
                //         .collect();
                //     println!("{} {:?}", param.name, s);
                // }
                // eprintln!("{:?}", callable.return_type);
                // callable.
            }
            // if field.callable.is_none() {
            //     eprintln!("{:?}", field.r#type);
            // }
        }
    }

    for type_ in types {
        eprintln!("{}", type_)
    }

    Ok(())
}
