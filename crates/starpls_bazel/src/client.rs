use std::{
    path::{Path, PathBuf},
    process::Command,
};

pub trait BazelClient {
    fn build_language(&self) -> anyhow::Result<Vec<u8>>;
}

pub struct BazelCLI {
    executable: PathBuf,
}

impl BazelCLI {
    pub fn new(executable: impl AsRef<Path>) -> Self {
        Self {
            executable: executable.as_ref().to_path_buf(),
        }
    }

    pub fn build_language(&self) -> anyhow::Result<Vec<u8>> {
        let output = Command::new(&self.executable)
            .args(["info", "build-language"])
            .output()?;
        Ok(output.stdout)
    }
}

impl Default for BazelCLI {
    fn default() -> Self {
        Self {
            executable: "bazel".into(),
        }
    }
}
