use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    builtin::{Callable, Param, Value},
    Builtins,
};

#[derive(Debug, Serialize, Deserialize)]
struct BuiltinsJson {
    builtins: Vec<ValueJson>,
}

impl From<BuiltinsJson> for Builtins {
    fn from(value: BuiltinsJson) -> Self {
        Self {
            global: value
                .builtins
                .into_iter()
                .map(|value| value.into())
                .collect(),
            ..Default::default()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ValueJson {
    name: String,
    doc: String,
    callable: Option<CallableJson>,
}

impl From<ValueJson> for Value {
    fn from(val: ValueJson) -> Self {
        Value {
            name: val.name,
            doc: val.doc,
            callable: val.callable.map(|callable| Callable {
                param: callable
                    .params
                    .into_iter()
                    .map(|param| Param {
                        name: param.name,
                        r#type: param.r#type,
                        doc: param.doc,
                        default_value: param.default_value,
                        is_mandatory: param.is_mandatory,
                        is_star_arg: param.is_star_arg,
                        is_star_star_arg: param.is_star_star_arg,
                    })
                    .collect(),
                return_type: callable.return_type,
            }),
            ..Default::default()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct CallableJson {
    params: Vec<ParamJson>,
    return_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ParamJson {
    name: String,
    r#type: String,
    doc: String,
    default_value: String,
    is_mandatory: bool,
    is_star_arg: bool,
    is_star_star_arg: bool,
}

/// The builtin.pb file is missing `module_extension`, `repository_rule` and `tag_class`.
pub fn make_bzl_builtins() -> Builtins {
    serde_json::from_str::<BuiltinsJson>(include_str!("../data/bzl.builtins.json"))
        .expect("bug: invalid bzl.builtins.json")
        .into()
}

pub fn make_build_builtins() -> Builtins {
    serde_json::from_str::<BuiltinsJson>(include_str!("../data/build.builtins.json"))
        .expect("bug: invalid build.builtins.json")
        .into()
}

pub fn make_module_bazel_builtins() -> Builtins {
    serde_json::from_str::<BuiltinsJson>(include_str!("../data/module-bazel.builtins.json"))
        .expect("bug: invalid module-bazel.builtins.json")
        .into()
}

pub fn make_workspace_builtins() -> Builtins {
    serde_json::from_str::<BuiltinsJson>(include_str!("../data/workspace.builtins.json"))
        .expect("bug: invalid workspace.builtins.json")
        .into()
}

pub fn make_repo_builtins() -> Builtins {
    serde_json::from_str::<BuiltinsJson>(include_str!("../data/repo.builtins.json"))
        .expect("bug: invalid repo.builtins.json")
        .into()
}

pub fn make_missing_module_members() -> HashMap<String, Vec<Value>> {
    serde_json::from_str::<HashMap<String, Vec<ValueJson>>>(include_str!(
        "../data/missingModuleFields.json"
    ))
    .expect("bug: invalid missingModuleFields.json")
    .into_iter()
    .map(|(name, fields)| (name, fields.into_iter().map(|field| field.into()).collect()))
    .collect()
}
