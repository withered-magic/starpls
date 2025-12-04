use lsp_types::ClientCapabilities;
use serde::Deserialize;
use serde_json::Value;

use crate::commands::server::ServerCommand;

#[derive(Deserialize, Default)]
#[serde(default)]
pub(crate) struct BuildifierConfig {
    pub(crate) path: Option<String>,
    pub(crate) args: Vec<String>,
}

#[derive(Deserialize)]
struct InitializationOptions {
    buildifier: Option<BuildifierConfig>,
}

#[derive(Default)]
pub(crate) struct ServerConfig {
    pub(crate) buildifier: Option<BuildifierConfig>,
    pub(crate) args: ServerCommand,
    pub(crate) caps: ClientCapabilities,
}

macro_rules! try_or_default {
    ($expr:expr) => {
        (|| $expr)().unwrap_or_default()
    };
}

impl ServerConfig {
    pub(crate) fn from_json(value: Value) -> Self {
        let mut config = ServerConfig::default();
        if let Ok(opts) = serde_json::from_value::<InitializationOptions>(value) {
            config.buildifier = opts.buildifier;
        }
        config
    }
    pub(crate) fn has_text_document_definition_link_support(&self) -> bool {
        try_or_default!(self.caps.text_document.as_ref()?.definition?.link_support)
    }

    pub(crate) fn has_insert_replace_support(&self) -> bool {
        try_or_default!(
            self.caps
                .text_document
                .as_ref()?
                .completion
                .as_ref()?
                .completion_item
                .as_ref()?
                .insert_replace_support
        )
    }
}
