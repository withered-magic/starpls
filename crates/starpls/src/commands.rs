use clap::Args;

pub(crate) mod check;
pub(crate) mod server;

#[derive(Args, Default)]
pub(crate) struct InferenceOptions {
    /// Infer attributes on a rule implementation function's context parameter.
    #[clap(long = "experimental_infer_ctx_attributes", default_value_t = false)]
    pub(crate) infer_ctx_attributes: bool,

    /// Use code-flow analysis during typechecking.
    #[clap(long = "experimental_use_code_flow_analysis", default_value_t = false)]
    pub(crate) use_code_flow_analysis: bool,
}
