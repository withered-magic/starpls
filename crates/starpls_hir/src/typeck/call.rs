//! A module providing facilities for validating arguments to function calls.
//! The routine implemented here is based on PEP 3102 (https://peps.python.org/pep-3102),
//! but with a couple of modifications for handling "*args" and "**kwargs" arguments.
use crate::{
    def::{Argument, Param},
    typeck::{builtins::BuiltinFunctionParam, intrinsics::IntrinsicFunctionParam},
    ExprId, Name, Ty,
};
use smallvec::{smallvec, SmallVec};

pub(crate) struct ArgError {
    pub(crate) expr: ExprId,
    pub(crate) message: String,
}

pub(crate) struct Slots(SmallVec<[Slot; 5]>);

impl Slots {
    pub(crate) fn assign_args(&mut self, args_with_ty: &[(Argument, Ty)]) -> Vec<ArgError> {
        let mut errors = Vec::new();

        'outer: for (arg, arg_ty) in args_with_ty {
            match arg {
                Argument::Simple { expr } => {
                    // Look for a positional/keyword parameter with no provider, or for a
                    // "*args" parameter.
                    let provider = SlotProvider::Single(*expr, arg_ty.clone());
                    for slot in self.0.iter_mut() {
                        match slot {
                            Slot::Positional {
                                provider: provider2 @ SlotProvider::Missing,
                            }
                            | Slot::Keyword {
                                provider: provider2 @ SlotProvider::Missing,
                                positional: true,
                                ..
                            } => {
                                *provider2 = provider;
                                continue 'outer;
                            }
                            Slot::ArgsList { providers } => {
                                providers.push(provider);
                                continue 'outer;
                            }
                            _ => {}
                        }
                    }

                    errors.push(ArgError {
                        expr: *expr,
                        message: "Unexpected positional argument".to_string(),
                    });
                }
                Argument::Keyword {
                    name: ref arg_name,
                    expr,
                } => {
                    // Look for either a keyword parameter matching this argument's
                    // name, or for the "**kwargs" parameter.
                    let provider = SlotProvider::Single(*expr, arg_ty.clone());
                    for slot in self.0.iter_mut() {
                        match slot {
                            Slot::Keyword {
                                name,
                                provider:
                                    provider2 @ (SlotProvider::Missing | SlotProvider::KwargsDict(_, _)),
                                ..
                            } if arg_name == name => {
                                *provider2 = provider;
                                continue 'outer;
                            }
                            Slot::ArgsList { providers } => {
                                providers.push(provider);
                                continue 'outer;
                            }
                            _ => {}
                        }
                    }

                    errors.push(ArgError {
                        expr: *expr,
                        message: format!("Unexpected keyword argument \"{}\"", arg_name.as_str()),
                    });
                }
                Argument::UnpackedList { expr } => {
                    // Mark all unfilled positional slots as well as the "*args" slot as being
                    // provided by this argument.
                    for slot in self.0.iter_mut() {
                        match slot {
                            Slot::Positional {
                                provider: provider @ SlotProvider::Missing,
                            } => *provider = SlotProvider::ArgsList(*expr, arg_ty.clone()),
                            Slot::ArgsList { providers } => {
                                providers.push(SlotProvider::ArgsList(*expr, arg_ty.clone()));
                            }
                            _ => {}
                        }
                    }
                }
                Argument::UnpackedDict { expr } => {
                    // Mark all keyword slots as well as the "**kwargs" slot as being provided by
                    // this argument.
                    for slot in self.0.iter_mut() {
                        match slot {
                            Slot::Keyword { provider, .. } => {
                                *provider = SlotProvider::KwargsDict(*expr, arg_ty.clone())
                            }
                            Slot::KwargsDict { providers } => {
                                providers.push(SlotProvider::KwargsDict(*expr, arg_ty.clone()))
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        errors
    }

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
        positional: bool,
    },
    ArgsList {
        providers: SmallVec<[SlotProvider; 1]>,
    },
    KwargsDict {
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
    KwargsDict(ExprId, Ty),
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
                Param::Simple { name, .. } => {
                    if saw_vararg {
                        break;
                    }

                    Slot::Keyword {
                        name: name.clone(),
                        provider: SlotProvider::Missing,
                        positional: !(saw_vararg || saw_kwargs),
                    }
                }
                Param::ArgsList { .. } => {
                    saw_vararg = true;
                    Slot::ArgsList {
                        providers: smallvec![],
                    }
                }
                Param::KwargsDict { .. } => {
                    saw_kwargs = true;
                    Slot::KwargsDict {
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
                    positional: false,
                },
                IntrinsicFunctionParam::ArgsList { .. } => {
                    saw_vararg = true;
                    Slot::ArgsList {
                        providers: smallvec![],
                    }
                }
                IntrinsicFunctionParam::KwargsDict => {
                    saw_kwargs = true;
                    Slot::KwargsDict {
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
                    name, positional, ..
                } => {
                    if saw_vararg {
                        break;
                    }

                    Slot::Keyword {
                        name: name.clone(),
                        provider: SlotProvider::Missing,
                        positional: *positional,
                    }
                }
                BuiltinFunctionParam::ArgsList { .. } => {
                    saw_vararg = true;
                    Slot::ArgsList {
                        providers: smallvec![],
                    }
                }
                BuiltinFunctionParam::KwargsDict => {
                    saw_kwargs = true;
                    Slot::KwargsDict {
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
