use lsp_types::ClientCapabilities;

pub(crate) struct ServerConfig {
    caps: ClientCapabilities,
}

macro_rules! try_or_default {
    ($expr:expr) => {
        (|| $expr)().unwrap_or_default()
    };
}

impl ServerConfig {
    pub(crate) fn new(caps: ClientCapabilities) -> Self {
        Self { caps }
    }

    pub(crate) fn has_text_document_definition_link_support(&self) -> bool {
        try_or_default!(self.caps.text_document.as_ref()?.definition?.link_support)
    }
}
