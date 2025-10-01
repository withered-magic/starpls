use std::path::Path;
use std::sync::Arc;

use starpls_bazel::APIContext;
use starpls_bazel::Builtins;

use crate::BuiltinProvider;
use crate::DialectDetector;
use crate::DialectId;
use crate::DialectInfo;
use crate::ExtensibleDialect;

/// Detector for Tiltfiles and Tilt-related Starlark files.
pub struct TiltDialectDetector;

impl DialectDetector for TiltDialectDetector {
    fn detect(&self, _workspace_path: &Path, file_path: &Path) -> Option<DialectInfo> {
        let basename = file_path.file_name()?.to_str()?;

        // Tilt uses specific file naming conventions
        match basename {
            "Tiltfile" => Some(DialectInfo {
                dialect_id: DialectId::new("tilt"),
                api_context: None, // Tilt doesn't use Bazel's API contexts
            }),
            name if name.ends_with(".tiltfile") => Some(DialectInfo {
                dialect_id: DialectId::new("tilt"),
                api_context: None,
            }),
            name if name.starts_with("Tiltfile.") => Some(DialectInfo {
                dialect_id: DialectId::new("tilt"),
                api_context: None,
            }),
            _ => None,
        }
    }

    fn priority(&self) -> u32 {
        150 // Higher priority than Bazel to catch Tiltfiles first
    }
}

/// Provider for Tilt builtin definitions.
pub struct TiltBuiltinProvider;

impl BuiltinProvider for TiltBuiltinProvider {
    fn load_builtins(&self, _api_context: Option<APIContext>) -> anyhow::Result<Builtins> {
        // This would normally load Tilt-specific builtin definitions
        // For this example, we'll create a minimal set
        let mut builtins = Builtins::default();

        // Add some example Tilt functions
        // In a real implementation, these would be loaded from JSON files
        // or generated from Tilt's own documentation
        let tilt_functions = create_example_tilt_builtins();
        builtins.global.extend(tilt_functions);

        Ok(builtins)
    }

    fn load_rules(&self, _api_context: Option<APIContext>) -> anyhow::Result<Builtins> {
        // Tilt doesn't have "rules" in the Bazel sense, but might have
        // resource types or extensions
        Ok(Builtins::default())
    }

    fn supported_contexts(&self) -> Vec<APIContext> {
        // Tilt doesn't use Bazel's API contexts
        vec![]
    }
}

/// Create example Tilt builtin functions for demonstration.
fn create_example_tilt_builtins() -> Vec<starpls_bazel::builtin::Value> {
    use starpls_bazel::builtin::Callable;
    use starpls_bazel::builtin::Param;
    use starpls_bazel::builtin::Value;

    vec![
        // docker_build function
        Value {
            name: "docker_build".to_string(),
            r#type: "function".to_string(),
            callable: Some(Callable {
                param: vec![
                    Param {
                        name: "ref".to_string(),
                        r#type: "string".to_string(),
                        doc: "The name for the image to be built.".to_string(),
                        default_value: "".to_string(),
                        is_mandatory: true,
                        is_star_arg: false,
                        is_star_star_arg: false,
                    },
                    Param {
                        name: "context".to_string(),
                        r#type: "string".to_string(),
                        doc: "The build context for the Docker image.".to_string(),
                        default_value: ".".to_string(),
                        is_mandatory: false,
                        is_star_arg: false,
                        is_star_star_arg: false,
                    },
                    Param {
                        name: "dockerfile".to_string(),
                        r#type: "string".to_string(),
                        doc: "Path to the Dockerfile relative to the context.".to_string(),
                        default_value: "Dockerfile".to_string(),
                        is_mandatory: false,
                        is_star_arg: false,
                        is_star_star_arg: false,
                    },
                ],
                return_type: "None".to_string(),
            }),
            doc: "Builds a Docker image.".to_string(),
            api_context: Default::default(), // Not used by Tilt
        },
        // k8s_yaml function
        Value {
            name: "k8s_yaml".to_string(),
            r#type: "function".to_string(),
            callable: Some(Callable {
                param: vec![Param {
                    name: "yaml".to_string(),
                    r#type: "string or list of strings".to_string(),
                    doc: "Path(s) to Kubernetes YAML files.".to_string(),
                    default_value: "".to_string(),
                    is_mandatory: true,
                    is_star_arg: false,
                    is_star_star_arg: false,
                }],
                return_type: "None".to_string(),
            }),
            doc: "Applies Kubernetes YAML to the cluster.".to_string(),
            api_context: Default::default(),
        },
        // local_resource function
        Value {
            name: "local_resource".to_string(),
            r#type: "function".to_string(),
            callable: Some(Callable {
                param: vec![
                    Param {
                        name: "name".to_string(),
                        r#type: "string".to_string(),
                        doc: "Name of the local resource.".to_string(),
                        default_value: "".to_string(),
                        is_mandatory: true,
                        is_star_arg: false,
                        is_star_star_arg: false,
                    },
                    Param {
                        name: "cmd".to_string(),
                        r#type: "string or list of strings".to_string(),
                        doc: "Command to run.".to_string(),
                        default_value: "".to_string(),
                        is_mandatory: true,
                        is_star_arg: false,
                        is_star_star_arg: false,
                    },
                    Param {
                        name: "deps".to_string(),
                        r#type: "list of strings".to_string(),
                        doc: "Files that this resource depends on.".to_string(),
                        default_value: "[]".to_string(),
                        is_mandatory: false,
                        is_star_arg: false,
                        is_star_star_arg: false,
                    },
                ],
                return_type: "None".to_string(),
            }),
            doc: "Defines a local resource that runs a command.".to_string(),
            api_context: Default::default(),
        },
    ]
}

/// Create a complete Tilt dialect definition.
pub fn create_tilt_dialect() -> ExtensibleDialect {
    ExtensibleDialect::new(
        DialectId::new("tilt"),
        "Tilt".to_string(),
        "A multi-service development environment for teams that deploy to Kubernetes".to_string(),
        Arc::new(TiltDialectDetector),
        Arc::new(TiltBuiltinProvider),
    )
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_tilt_detector() {
        let detector = TiltDialectDetector;
        let workspace = PathBuf::from("/workspace");

        // Test main Tiltfile
        let tiltfile = PathBuf::from("/workspace/Tiltfile");
        let info = detector.detect(&workspace, &tiltfile).unwrap();
        assert_eq!(info.dialect_id, DialectId::new("tilt"));
        assert_eq!(info.api_context, None);

        // Test environment-specific Tiltfile
        let env_tiltfile = PathBuf::from("/workspace/Tiltfile.dev");
        let info = detector.detect(&workspace, &env_tiltfile).unwrap();
        assert_eq!(info.dialect_id, DialectId::new("tilt"));

        // Test .tiltfile extension
        let ext_tiltfile = PathBuf::from("/workspace/config.tiltfile");
        let info = detector.detect(&workspace, &ext_tiltfile).unwrap();
        assert_eq!(info.dialect_id, DialectId::new("tilt"));

        // Test non-Tilt file
        let other_file = PathBuf::from("/workspace/BUILD");
        assert!(detector.detect(&workspace, &other_file).is_none());
    }

    #[test]
    fn test_tilt_builtin_provider() {
        let provider = TiltBuiltinProvider;

        // Test loading builtins
        let builtins = provider.load_builtins(None).unwrap();
        assert!(!builtins.global.is_empty());

        // Check that we have the expected Tilt functions
        let function_names: Vec<_> = builtins.global.iter().map(|f| &f.name).collect();
        assert!(function_names.contains(&&"docker_build".to_string()));
        assert!(function_names.contains(&&"k8s_yaml".to_string()));
        assert!(function_names.contains(&&"local_resource".to_string()));

        // Test supported contexts
        let contexts = provider.supported_contexts();
        assert!(contexts.is_empty()); // Tilt doesn't use Bazel contexts
    }

    #[test]
    fn test_tilt_function_signatures() {
        let provider = TiltBuiltinProvider;
        let builtins = provider.load_builtins(None).unwrap();

        // Find docker_build function and verify its signature
        let docker_build = builtins
            .global
            .iter()
            .find(|f| f.name == "docker_build")
            .expect("docker_build function should exist");

        let callable = docker_build
            .callable
            .as_ref()
            .expect("docker_build should be callable");

        assert_eq!(callable.param.len(), 3);
        assert_eq!(callable.param[0].name, "ref");
        assert!(callable.param[0].is_mandatory);
        assert_eq!(callable.param[1].name, "context");
        assert!(!callable.param[1].is_mandatory);
    }
}
