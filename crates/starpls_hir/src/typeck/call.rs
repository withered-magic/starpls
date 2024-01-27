//! A module providing facilities for validating arguments to function calls.
//! The routine implemented here is based on PEP 3102 (https://peps.python.org/pep-3102),
//! but with a couple of modifications for handling "*args" and "**kwargs" arguments.
use crate::{
    def::Param,
    typeck::{builtins::BuiltinFunctionParam, intrinsics::IntrinsicFunctionParam},
    ExprId, Name, Ty,
};
use smallvec::{smallvec, SmallVec};

pub(crate) struct Slots(SmallVec<[Slot; 5]>);

impl Slots {
    pub(crate) fn iter_mut(&mut self) -> impl Iterator<Item = &mut Slot> {
        self.0.iter_mut()
    }

    pub(crate) fn into_inner(self) -> SmallVec<[Slot; 5]> {
        self.0
    }
}

/// A slot for a formal parameter, as defined in PEP 3102.
pub(crate) enum Slot {
    Positional {
        provider: SlotProvider,
    },
    Keyword {
        name: Name,
        provider: SlotProvider,
    },
    ArgsList {
        providers: SmallVec<[SlotProvider; 1]>,
    },
    KwargsList {
        providers: SmallVec<[SlotProvider; 1]>,
    },
}

/// Describes a value assigned to a slot. This type enumerates
/// all the ways in which arguments can be passed to a function.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum SlotProvider {
    Missing,
    Single(ExprId, Ty),
    ArgsList(ExprId, Ty),
    KwargsList(ExprId, Ty),
}

impl From<&[Param]> for Slots {
    fn from(params: &[Param]) -> Self {
        let mut slots = smallvec![];
        let mut saw_vararg = false;
        let mut saw_kwargs = false;

        // Derive slots only from the valid formal parameters.
        // For example, don't match a second `**kwargs` parameter.
        for param in params.iter() {
            let slot = match param {
                Param::Simple {
                    name,
                    default,
                    type_ref,
                } => {}
                Param::ArgsList { name, type_ref } => todo!(),
                Param::KwargsList { name, type_ref } => todo!(),
            };
        }

        Self(slots)
    }
}

impl From<&[IntrinsicFunctionParam]> for Slots {
    fn from(params: &[IntrinsicFunctionParam]) -> Self {
        let mut slots = smallvec![];
        let mut saw_vararg = false;
        let mut saw_kwargs = false;

        // Derive slots only from the valid formal parameters.
        // For example, don't match a second `**kwargs` parameter.
        for param in params.iter() {
            let slot = match param {
                IntrinsicFunctionParam::Positional { .. } => {
                    if saw_vararg {
                        // TODO: Emit diagnostics for invalid parameters.
                        break;
                    }
                    Slot::Positional {
                        provider: SlotProvider::Missing,
                    }
                }
                IntrinsicFunctionParam::Keyword { name, .. } => Slot::Keyword {
                    name: name.clone(),
                    provider: SlotProvider::Missing,
                },
                IntrinsicFunctionParam::ArgsList { .. } => {
                    saw_vararg = true;
                    Slot::ArgsList {
                        providers: smallvec![],
                    }
                }
                IntrinsicFunctionParam::KwargsList => {
                    saw_kwargs = true;
                    Slot::KwargsList {
                        providers: smallvec![],
                    }
                }
            };

            slots.push(slot);

            // Nothing can follow a `**kwargs` parameter.
            if saw_kwargs {
                break;
            }
        }

        Self(slots)
    }
}

impl From<&[BuiltinFunctionParam]> for Slots {
    fn from(params: &[BuiltinFunctionParam]) -> Self {
        // See the implementation for `IntrinsicFunctionParam` above.
        let mut slots = smallvec![];
        let mut saw_vararg = false;
        let mut saw_kwargs = false;

        // Derive slots only from the valid formal parameters.
        // For example, don't match a second `**kwargs` parameter.
        for param in params.iter() {
            let slot = match param {
                BuiltinFunctionParam::Simple {
                    name,
                    type_ref,
                    default_value,
                    positional,
                } => todo!(),
                BuiltinFunctionParam::ArgsList { type_ref } => todo!(),
                BuiltinFunctionParam::KwargsList => todo!(),
            };

            slots.push(slot);

            // Nothing can follow a `**kwargs` parameter.
            if saw_kwargs {
                break;
            }
        }

        Self(slots)
    }
}
