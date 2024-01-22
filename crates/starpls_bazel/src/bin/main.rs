use starpls_bazel::load_builtins;
use std::collections::HashSet;

fn main() -> anyhow::Result<()> {
    let builtins = load_builtins("builtin.pb")?;
    let types: HashSet<_> = builtins
        .r#type
        .iter()
        .map(|type_| type_.name.clone())
        .collect();

    for builtin in builtins.global {
        println!("{:?} {:?}", builtin.name, builtin.r#type);
    }

    // for builtin in builtins.r#type.iter() {
    //     for field in builtin.field.iter() {
    //         if let Some(ref callable) = field.callable {
    //             // for param in callable.param.iter() {
    //             //     let mut s = String::new();
    //             //     let mut chars = param.r#type.chars();
    //             //     let mut in_tag = true;
    //             //     while let Some(c) = chars.next() {
    //             //         match (c, in_tag) {
    //             //             ('<', false) => in_tag = true,
    //             //             ('>', true) => in_tag = false,
    //             //             (c, false) => s.push(c),
    //             //             _ => {}
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

    Ok(())
}
