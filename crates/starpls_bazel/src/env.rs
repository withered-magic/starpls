use crate::{
    builtin::{Callable, Param, Value},
    Builtins,
};

/// The builtin.pb file is missing `module_extension`, `repository_rule` and `tag_class`.
pub fn make_bzl_builtins() -> Builtins {
    Builtins {
        r#type: vec![],
        global: vec![
            Value {
                name: "module_extension".to_string(),
                doc: "Creates a new module extension. Store it in a global value, so that it can be exported and used in a MODULE.bazel file with <code>use_extension</code>.".to_string(),
                callable: Some(Callable {
                    param: vec![
                        Param {
                            name: "implementation".to_string(),
                            r#type: "Unknown".to_string(),
                            doc: "The function that implements this module extension. Must take a single parameter, <code>module_ctx</code>. The function is called once at the beginning of a build to determine the set of available repos.".to_string(),
                            default_value: "".to_string(),
                            is_mandatory: true,
                            is_star_arg: false,
                            is_star_star_arg: false,
                        },
                        Param {
                            name: "tag_classes".to_string(),
                            r#type: "Unknown".to_string(),
                            doc: "A dictionary to declare all the tag classes used by the extension. It maps from the name of the tag class to a <code>tag_class</code> object.".to_string(),
                            default_value: "{}".to_string(),
                            is_mandatory: false,
                            is_star_arg: false,
                            is_star_star_arg: false,
                        },
                        Param {
                            name: "doc".to_string(),
                            r#type: "string; or None".to_string(),
                            doc: "A description of the module extension that can be extracted by documentation generating tools.".to_string(),
                            default_value: "None".to_string(),
                            is_mandatory: false,
                            is_star_arg: false,
                            is_star_star_arg: false,
                        },
                        Param {
                            name: "environ".to_string(),
                            r#type: "sequence of strings".to_string(),
                            doc: "Provides a list of environment variable that this module extension depends on. If an environment variable in that list changes, the extension will be re-evaluated.".to_string(),
                            default_value: "[]".to_string(),
                            is_mandatory: false,
                            is_star_arg: false,
                            is_star_star_arg: false,
                        },
                        Param {
                            name: "os_dependent".to_string(),
                            r#type: "boolean".to_string(),
                            doc: "Indicates whether this extension is OS-dependent or not".to_string(),
                            default_value: "False".to_string(),
                            is_mandatory: false,
                            is_star_arg: false,
                            is_star_star_arg: false,
                        },
                        Param {
                            name: "arch_dependent".to_string(),
                            r#type: "boolean".to_string(),
                            doc: "Indicates whether this extension is architecture-dependent or not".to_string(),
                            default_value: "False".to_string(),
                            is_mandatory: false,
                            is_star_arg: false,
                            is_star_star_arg: false,
                        }
                    ],
                    return_type: "Unknown".to_string(),
                }),
                ..Default::default()
            },
            Value {
                name: "repository_rule".to_string(),
                doc: "Creates a new repository rule. Store it in a global value, so that it can be loaded and called from a <code>module extension</code> implementation function, or used by <code>use_repo_rule</code>.".to_string(),
                callable: Some(Callable {
                    param: vec![
                        Param {
                            name: "implementation".to_string(),
                            r#type: "Unknown".to_string(),
                            doc: "the function that implements this rule. Must have a single parameter, <code>repository_ctx</code>. The function is called during the loading phase for each instance of the rule.".to_string(),
                            default_value: "".to_string(),
                            is_mandatory: true,
                            is_star_arg: false,
                            is_star_star_arg: false,
                        },
                        Param {
                            name: "attrs".to_string(),
                            r#type: "dict; or None".to_string(),
                            doc: "dictionary to declare all the attributes of the rule. It maps from an attribute name to an attribute object (see attr module). Attributes starting with _ are private, and can be used to add an implicit dependency on a label to a file (a repository rule cannot depend on a generated artifact). The attribute name is implicitly added and must not be specified.".to_string(),
                            default_value: "None".to_string(),
                            is_mandatory: false,
                            is_star_arg: false,
                            is_star_star_arg: false,
                        },
                        Param {
                            name: "local".to_string(),
                            r#type: "boolean".to_string(),
                            doc: "Indicate that this rule fetches everything from the local system and should be reevaluated at every fetch.".to_string(),
                            default_value: "False".to_string(),
                            is_mandatory: false,
                            is_star_arg: false,
                            is_star_star_arg: false,
                        },
                        Param {
                            name: "environ".to_string(),
                            r#type: "sequence of strings".to_string(),
                            doc: "Deprecated. This parameter has been deprecated. Migrate to <code>repository_ctx.getenv</code> instead. Provides a list of environment variable that this repository rule depends on. If an environment variable in that list change, the repository will be refetched.".to_string(),
                            default_value: "[]".to_string(),
                            is_mandatory: false,
                            is_star_arg: false,
                            is_star_star_arg: false,
                        },
                        Param {
                            name: "configure".to_string(),
                            r#type: "boolean".to_string(),
                            doc: "Indicate that the repository inspects the system for configuration purpose".to_string(),
                            default_value: "False".to_string(),
                            is_mandatory: false,
                            is_star_arg: false,
                            is_star_star_arg: false,
                        },
                        Param {
                            name: "remotable".to_string(),
                            r#type: "boolean".to_string(),
                            doc: "Experimental. This parameter is experimental and may change at any time. Please do not depend on it. It may be enabled on an experimental basis by setting <code>---experimental_repo_remote_exec</code>. Compatible with remote execution".to_string(),
                            default_value: "False".to_string(),
                            is_mandatory: false,
                            is_star_arg: false,
                            is_star_star_arg: false,
                        },
                        Param {
                            name: "doc".to_string(),
                            r#type: "string; or None".to_string(),
                            doc: "A description of the repository rule that can be extracted by documentation generating tools.".to_string(),
                            default_value: "None".to_string(),
                            is_mandatory: false,
                            is_star_arg: false,
                            is_star_star_arg: false,
                        },
                    ],
                    return_type: "callable".to_string(),
                }),
                ..Default::default()
            },
            Value {
                name: "tag_class".to_string(),
                doc: "Creates a new <code>tag_class</code> object, which defines an attribute schema for a class of tags, which are data objects usable by a module extension.".to_string(),
                callable: Some(Callable {
                    param: vec![
                        Param {
                            name: "attrs".to_string(),
                            r#type: "dict".to_string(),
                            doc: "A dictionary to declare all the attributes of this tag class. It maps from an attribute name to an attribute object (see attr module).".to_string(),
                            default_value: "{}".to_string(),
                            is_mandatory: false,
                            is_star_arg: false,
                            is_star_star_arg: false,
                        },
                        Param {
                            name: "doc".to_string(),
                            r#type: "string; or None".to_string(),
                            doc: "A description of the tag class that can be extracted by documentation generating tools.".to_string(),
                            default_value: "None".to_string(),
                            is_mandatory: false,
                            is_star_arg: false,
                            is_star_star_arg: false,
                        },
                    ],
                    return_type: "tag_class".to_string(),
                }),
                ..Default::default()
            }
        ],
    }
}
