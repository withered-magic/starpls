use std::{
    path::{Path, PathBuf},
    process::Command,
};

pub trait BazelClient: Send + Sync + 'static {
    fn build_language(&self) -> anyhow::Result<Vec<u8>>;
    fn output_base(&self) -> anyhow::Result<PathBuf>;
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

    fn run_command(&self, args: &[&str]) -> anyhow::Result<Vec<u8>> {
        let output = Command::new(&self.executable).args(args).output()?;
        Ok(output.stdout)
    }
}

impl BazelClient for BazelCLI {
    fn build_language(&self) -> anyhow::Result<Vec<u8>> {
        self.run_command(&["info", "build-language"])
    }

    fn output_base(&self) -> anyhow::Result<PathBuf> {
        let output = self.run_command(&["info", "output_base"])?;
        Ok(String::from_utf8(output)?.trim().into())
    }
}

impl Default for BazelCLI {
    fn default() -> Self {
        Self {
            executable: "bazel".into(),
        }
    }
}
