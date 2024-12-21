use anyhow::anyhow;
use starpls_bazel::client::BazelClient;
use starpls_bazel::client::BazelInfo;
use starpls_bazel::Builtins;

use crate::server::load_bazel_build_language;
use crate::server::load_bazel_builtins;

#[derive(Default)]
pub(crate) struct BazelContext {
    pub(crate) info: BazelInfo,
    pub(crate) builtins: Builtins,
    pub(crate) rules: Builtins,
    pub(crate) bzlmod_enabled: bool,
}

impl BazelContext {
    pub(crate) fn new(client: &dyn BazelClient) -> anyhow::Result<BazelContext> {
        let info = client.info()?;

        // We determine whether to use bzlmod in two steps. First, we check if `MODULE.bazel` exists at all,
        // and if so, whether the `bazel mod dump_repo_mapping` command is supported. If either of these
        // checks fails, then we can't use bzlmod anyways.
        let bzlmod_capability = info
            .workspace
            .join("MODULE.bazel")
            .try_exists()
            .unwrap_or(false)
            && { client.dump_repo_mapping("").is_ok() };

        let bzlmod_enabled = bzlmod_capability && {
            // Next, we check if bzlmod is enabled by default for the current Bazel version.
            // bzlmod is enabled by default for Bazel versions 7 and later.
            // TODO(withered-magic): Just hardcoding this for now since I'm lazy to parse the actual versions.
            // This should last us pretty long since Bazel 9 isn't anywhere on the horizon.
            let bzlmod_enabled_by_default = ["release 7", "release 8", "release 9"]
                .iter()
                .any(|release| info.release.starts_with(release));

            // Finally, check starlark-semantics to determine whether bzlmod has been explicitly
            // enabled/disabled, e.g. in a .bazelrc file.
            if info.starlark_semantics.contains("enable_bzlmod=true") {
                true
            } else if info.starlark_semantics.contains("enable_bzlmod=false") {
                false
            } else {
                bzlmod_enabled_by_default
            }
        };

        Ok(BazelContext {
            info,
            builtins: load_bazel_builtins()
                .map_err(|err| anyhow!("failed to load builtins: {}", err))?,
            rules: load_bazel_build_language(client)
                .map_err(|err| anyhow!("failed to run `bazel info build-language`: {}", err))?,
            bzlmod_enabled,
        })
    }
}
