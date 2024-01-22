use starpls_bazel::load_builtins;

fn main() -> anyhow::Result<()> {
    let builtins = load_builtins("builtin.pb")?;
    for builtin in builtins.r#type {
        println!("{:?}", builtin.name);
    }
    Ok(())
}
