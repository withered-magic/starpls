use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::str;

use anyhow::anyhow;
use anyhow::bail;
use parking_lot::RwLock;
use serde_json::Deserializer;

const DEFAULT_WORKSPACE_NAMES: &[&str] = &["__main__", "_main"];

#[derive(Default)]
pub struct BazelInfo {
    pub output_base: PathBuf,
    pub release: String,
    pub starlark_semantics: String,
    pub workspace: PathBuf,
    pub workspace_name: Option<String>,
}

pub trait BazelClient: Send + Sync + 'static {
    fn build_language(&self) -> anyhow::Result<Vec<u8>>;
    fn info(&self) -> anyhow::Result<BazelInfo>;
    fn resolve_repo_from_mapping(
        &self,
        apparent_repo: &str,
        from_repo: &str,
    ) -> anyhow::Result<Option<String>>;
    fn clear_repo_mappings(&self);
    fn null_query_external_repo_targets(&self, repo: &str) -> anyhow::Result<()>;
    fn repo_mapping_keys(&self, from_repo: &str) -> anyhow::Result<Vec<String>>;
    fn query_all_workspace_targets(&self) -> anyhow::Result<Vec<String>>;
    fn fetch_repo(&self, repo: &str) -> anyhow::Result<()>;
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

    fn run_command<I, S>(&self, args: I) -> anyhow::Result<Vec<u8>>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<std::ffi::OsStr>,
    {
        let output = Command::new(&self.executable).args(args).output()?;
        if !output.status.success() {
            bail!(
                "failed to run Bazel command with exit status {}, stderr={:?}",
                output.status,
                str::from_utf8(&output.stderr)?,
            );
        }
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

    fn info(&self) -> anyhow::Result<BazelInfo> {
        let output = self.run_command(&[
            "info",
            "execution_root",
            "output_base",
            "release",
            "starlark-semantics",
            "workspace",
        ])?;

        let output = str::from_utf8(&output)?;
        let mut output_base = None;
        let mut release = None;
        let mut starlark_semantics = None;
        let mut workspace = None;
        let mut workspace_name = None;
        for line in output.lines() {
            let (key, value) = match line.split_once(": ") {
                Some(pair) => pair,
                None => continue,
            };
            match key {
                "execution_root" => {
                    // Taken from https://github.com/cameron-martin/bazel-lsp/blob/92644f21aca7cfbba332c67ac1aa9cf43765e021/src/workspace.rs#L24.
                    workspace_name = PathBuf::from(value).file_name().and_then(|file_name| {
                        match file_name.to_string_lossy().to_string() {
                            name if DEFAULT_WORKSPACE_NAMES.contains(&name.as_str()) => None,
                            name => Some(name),
                        }
                    });
                }
                "output_base" => output_base = Some(value),
                "release" => release = Some(value),
                "starlark-semantics" => starlark_semantics = Some(value),
                "workspace" => workspace = Some(value),
                _ => {}
            }
        }

        Ok(BazelInfo {
            output_base: output_base
                .ok_or_else(|| anyhow!("failed to determine output_base from `bazel info`"))?
                .into(),
            release: release
                .ok_or_else(|| anyhow!("failed to determine release from `bazel info`"))?
                .into(),
            starlark_semantics: starlark_semantics
                .ok_or_else(|| anyhow!("failed to determine starlark-semantics from `bazel info`"))?
                .into(),
            workspace: workspace
                .ok_or_else(|| anyhow!("failed to determine workspace from `bazel info`"))?
                .into(),
            workspace_name,
        })
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

    fn clear_repo_mappings(&self) {
        self.repo_mappings.write().clear();
    }

    fn null_query_external_repo_targets(&self, repo: &str) -> anyhow::Result<()> {
        self.run_command(&["query", "--keep_going", &format!("@@{}//...", repo)])?;
        Ok(())
    }

    fn repo_mapping_keys(&self, from_repo: &str) -> anyhow::Result<Vec<String>> {
        let mappings = self.repo_mappings.read();
        if let Some(mapping) = mappings.get(from_repo) {
            return Ok(mapping.keys().cloned().collect());
        }
        drop(mappings);

        // Otherwise, fetch the repo mapping and cache it. For now, we always cache the result, even if the call failed.
        let mapping = self.dump_repo_mapping(from_repo).unwrap_or_default();
        let keys = mapping.keys().cloned().collect();
        self.repo_mappings
            .write()
            .insert(from_repo.to_string(), mapping);
        Ok(keys)
    }

    fn query_all_workspace_targets(&self) -> anyhow::Result<Vec<String>> {
        let output = self.run_command(["query", "kind('.* rule', ...)"])?;
        let targets = str::from_utf8(&output)?
            .lines()
            .map(|line| line.to_string())
            .collect();
        Ok(targets)
    }

    fn fetch_repo(&self, repo: &str) -> anyhow::Result<()> {
        self.run_command(["fetch", "--repo", &format!("@@{}", repo)])?;
        Ok(())
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
