use crate::{def::Declaration, Name};
use id_arena::{Arena, Id};
use rustc_hash::FxHashMap;

pub(crate) type ScopeId = Id<Scope>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum ScopeKind {
    Module,
    Function,
    ListComprehension,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Scope {
    pub(crate) kind: ScopeKind,
    pub(crate) declarations: FxHashMap<Name, Vec<Declaration>>,
    pub(crate) parent: Option<ScopeId>,
}

impl Scope {}

pub struct Scopes {
    scopes: Arena<Scope>,
}
