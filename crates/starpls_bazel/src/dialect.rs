use std::path::Path;

use anyhow;

// Temporary local trait definitions to avoid circular dependency
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct DialectId(pub String);

impl DialectId {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DialectInfo {
    pub dialect_id: DialectId,
    pub api_context: Option<crate::APIContext>,
}

pub trait DialectDetector: Send + Sync {
    fn detect(&self, workspace_path: &Path, file_path: &Path) -> Option<DialectInfo>;
    fn priority(&self) -> u32 {
        0
    }
}

pub trait BuiltinProvider: Send + Sync {
    fn load_builtins(
        &self,
        api_context: Option<crate::APIContext>,
    ) -> anyhow::Result<crate::Builtins>;
    fn load_rules(&self, api_context: Option<crate::APIContext>)
        -> anyhow::Result<crate::Builtins>;
    fn supported_contexts(&self) -> Vec<crate::APIContext>;
}

use crate::env;
use crate::APIContext;
use crate::Builtins;

/// Detector for Bazel Starlark files.
pub struct BazelDialectDetector;

impl DialectDetector for BazelDialectDetector {
    fn detect(&self, workspace_path: &Path, file_path: &Path) -> Option<DialectInfo> {
        let basename = file_path.file_name()?.to_str()?;
        let api_context = match basename {
            "BUILD" | "BUILD.bazel" => Some(APIContext::Build),
            "REPO.bazel" => Some(APIContext::Repo),
            "VENDOR.bazel" => Some(APIContext::Vendor),
            "MODULE.bazel" => Some(APIContext::Module),
            path if path.ends_with(".MODULE.bazel") => Some(APIContext::Module),
            "WORKSPACE" | "WORKSPACE.bazel" | "WORKSPACE.bzlmod" => Some(APIContext::Workspace),
            path if path.ends_with(".BUILD.bazel") || path.ends_with(".BUILD") => {
                Some(APIContext::Build)
            }
            path if path.ends_with(".cquery") || path.ends_with(".query.bzl") => {
                Some(APIContext::Cquery)
            }
            _ => match file_path.extension()?.to_str()? {
                "bzl" => Some(APIContext::Bzl),
                _ => {
                    // Check for prelude file
                    if file_path == workspace_path.join("tools/build_rules/prelude_bazel") {
                        Some(APIContext::Prelude)
                    } else {
                        return None; // Not a Bazel file
                    }
                }
            },
        };

        Some(DialectInfo {
            dialect_id: DialectId::new("bazel"),
            api_context,
        })
    }

    fn priority(&self) -> u32 {
        100 // High priority for Bazel files
    }
}

/// Provider for Bazel builtin definitions.
pub struct BazelBuiltinProvider;

impl BuiltinProvider for BazelBuiltinProvider {
    fn load_builtins(&self, api_context: Option<APIContext>) -> anyhow::Result<Builtins> {
        let builtins = match api_context {
            Some(APIContext::Bzl) => env::make_bzl_builtins(),
            Some(APIContext::Build) => env::make_build_builtins(),
            Some(APIContext::Module) => env::make_module_bazel_builtins(),
            Some(APIContext::Workspace) => env::make_workspace_builtins(),
            Some(APIContext::Repo) => env::make_repo_builtins(),
            Some(APIContext::Cquery) => env::make_cquery_builtins(),
            Some(APIContext::Vendor) => env::make_vendor_builtins(),
            Some(APIContext::Prelude) | None => {
                // Default to Bzl builtins for unknown contexts
                env::make_bzl_builtins()
            }
        };
        Ok(builtins)
    }

    fn load_rules(&self, _api_context: Option<APIContext>) -> anyhow::Result<Builtins> {
        // For now, return empty rules - this would be populated with actual rule definitions
        Ok(Builtins::default())
    }

    fn supported_contexts(&self) -> Vec<APIContext> {
        vec![
            APIContext::Bzl,
            APIContext::Build,
            APIContext::Module,
            APIContext::Workspace,
            APIContext::Repo,
            APIContext::Cquery,
            APIContext::Vendor,
            APIContext::Prelude,
        ]
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_bazel_detector() {
        let detector = BazelDialectDetector;
        let workspace = PathBuf::from("/workspace");

        // Test BUILD file
        let build_file = PathBuf::from("/workspace/BUILD");
        let info = detector.detect(&workspace, &build_file).unwrap();
        assert_eq!(info.dialect_id, DialectId::new("bazel"));
        assert_eq!(info.api_context, Some(APIContext::Build));

        // Test .bzl file
        let bzl_file = PathBuf::from("/workspace/rules.bzl");
        let info = detector.detect(&workspace, &bzl_file).unwrap();
        assert_eq!(info.api_context, Some(APIContext::Bzl));

        // Test non-Bazel file
        let other_file = PathBuf::from("/workspace/file.py");
        assert!(detector.detect(&workspace, &other_file).is_none());
    }

    #[test]
    fn test_bazel_builtin_provider() {
        let provider = BazelBuiltinProvider;

        // Test loading builtins for different contexts
        let bzl_builtins = provider.load_builtins(Some(APIContext::Bzl)).unwrap();
        assert!(!bzl_builtins.global.is_empty());

        let build_builtins = provider.load_builtins(Some(APIContext::Build)).unwrap();
        assert!(!build_builtins.global.is_empty());

        // Test supported contexts
        let contexts = provider.supported_contexts();
        assert!(contexts.contains(&APIContext::Bzl));
        assert!(contexts.contains(&APIContext::Build));
    }
}
