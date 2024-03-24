use std::io::Result;

fn main() -> Result<()> {
    prost_build::compile_protos(&["data/builtin.proto", "data/build.proto"], &["data/"])?;
    Ok(())
}
