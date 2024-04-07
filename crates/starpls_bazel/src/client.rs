use anyhow::anyhow;
use parking_lot::RwLock;
use serde_json::Deserializer;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    process::Command,
};

pub trait BazelClient: Send + Sync + 'static {
    fn build_language(&self) -> anyhow::Result<Vec<u8>>;
    fn output_base(&self) -> anyhow::Result<PathBuf>;
    fn workspace(&self) -> anyhow::Result<PathBuf>;
    fn resolve_repo_from_mapping(
        &self,
        apparent_repo: &str,
        from_repo: &str,
    ) -> anyhow::Result<Option<String>>;
}

pub struct BazelCLI {
    executable: PathBuf,
    repo_mappings: RwLock<HashMap<String, HashMap<String, String>>>,
}

impl BazelCLI {
    pub fn new(executable: impl AsRef<Path>) -> Self {
        Self {
            executable: executable.as_ref().to_path_buf(),
            ..Default::default()
        }
    }

    fn run_command(&self, args: &[&str]) -> anyhow::Result<Vec<u8>> {
        let output = Command::new(&self.executable).args(args).output()?;
        Ok(output.stdout)
    }

    pub fn dump_repo_mapping(&self, repo: &str) -> anyhow::Result<HashMap<String, String>> {
        let output = self.run_command(&["mod", "--enable_bzlmod", "dump_repo_mapping", repo])?;
        let json = String::from_utf8(output)?;
        let mut mappings = Deserializer::from_str(&json).into_iter::<HashMap<String, String>>();
        Ok(mappings
            .next()
            .ok_or_else(|| anyhow!("missing repo mapping for repository: {:?}", repo))??)
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

    fn workspace(&self) -> anyhow::Result<PathBuf> {
        let output = self.run_command(&["info", "workspace"])?;
        Ok(String::from_utf8(output)?.trim().into())
    }

    fn resolve_repo_from_mapping(
        &self,
        apparent_repo: &str,
        from_repo: &str,
    ) -> anyhow::Result<Option<String>> {
        // First, check if we've already fetched the repo mapping for the repository specified by `from_repo`.
        let mappings = self.repo_mappings.read();
        if let Some(mapping) = mappings.get(from_repo) {
            return Ok(mapping.get(apparent_repo).cloned());
        }
        drop(mappings);

        // Otherwise, fetch the repo mapping and cache it. For now, we always cache the result, even if the call failed.
        let mapping = self.dump_repo_mapping(from_repo).unwrap_or_default();
        let canonical_repo = mapping.get(apparent_repo).cloned();
        self.repo_mappings
            .write()
            .insert(from_repo.to_string(), mapping);
        Ok(canonical_repo)
    }
}

impl Default for BazelCLI {
    fn default() -> Self {
        Self {
            executable: "bazel".into(),
            repo_mappings: Default::default(),
        }
    }
}
