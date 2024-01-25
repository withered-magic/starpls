//! A module providing facilities for validating arguments to function calls.
//! The routine implemented here is based on PEP 3102 (https://peps.python.org/pep-3102),
//! but with a couple of modifications for handling "*args" and "**kwargs" arguments.
use crate::{
    typeck::{builtins::BuiltinFunctionParam, intrinsics::IntrinsicFunctionParam},
    ExprId, Name, Ty,
};
use smallvec::{smallvec, SmallVec};

pub(crate) struct Slots(SmallVec<[Slot; 5]>);

/// A slot for a formal parameter, as defined in PEP 3102.
enum Slot {
    Positional {
        provider: SlotProvider,
    },
    Keyword {
        name: Name,
        provider: SlotProvider,
    },
    VarArgList {
        providers: SmallVec<[SlotProvider; 1]>,
    },
    VarArgDict {
        providers: SmallVec<[SlotProvider; 1]>,
    },
}

/// Describes a value assigned to a slot. This type enumerates
/// all the ways in which arguments can be passed to a function.
#[derive(Clone, Debug, PartialEq, Eq)]
enum SlotProvider {
    Missing,
    Single(ExprId, Ty),
    VarArgList(ExprId, Ty),
    VarArgDict(ExprId, Ty),
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
                IntrinsicFunctionParam::VarArgList { .. } => {
                    saw_vararg = true;
                    Slot::VarArgList {
                        providers: smallvec![],
                    }
                }
                IntrinsicFunctionParam::VarArgDict => {
                    saw_kwargs = true;
                    Slot::VarArgDict {
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
                BuiltinFunctionParam::Normal {
                    name,
                    type_ref,
                    kw_only,
                    optional,
                } => todo!(),
                BuiltinFunctionParam::VarArgList { type_ref } => todo!(),
                BuiltinFunctionParam::VarArgDict => todo!(),
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
