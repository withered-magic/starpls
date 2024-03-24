use std::io::Result;

fn main() -> Result<()> {
    prost_build::compile_protos(
        &["src/builtin.proto", "data/build.proto"],
        &["data/", "src/"],
    )?;
    Ok(())
}
