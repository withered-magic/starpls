use std::mem;

use crate::{Output, SyntaxKind};

/// A step in the process of building a syntax tree.
#[derive(Debug)]
pub enum Step {
    Start { kind: SyntaxKind },
    Finish,
    Token { kind: SyntaxKind },
    Error { message: String },
}

/// Raw events produced by the processor. They contain additional fields, such
/// as `forward_parent`, and thus are unsuitable for direct usage; instead, we need
/// to run postprocessing steps to clean up the events.
#[derive(Debug)]
pub(crate) enum StepEvent {
    Start {
        kind: SyntaxKind,
        forward_parent: Option<u32>,
    },
    Finish,
    Token {
        kind: SyntaxKind,
    },
    Error {
        message: String,
    },
    Tombstone,
}

pub(super) fn postprocess_step_events(mut events: Vec<StepEvent>) -> Output {
    let mut steps = Vec::new();
    let mut forward_parent_kinds = Vec::new();

    for i in 0..events.len() {
        match mem::replace(&mut events[i], StepEvent::Tombstone) {
            StepEvent::Start {
                kind,
                forward_parent,
            } => {
                let mut fp = forward_parent;
                forward_parent_kinds.push(kind);
                while let Some(forward_parent_pos) = fp {
                    match mem::replace(
                        &mut events[forward_parent_pos as usize],
                        StepEvent::Tombstone,
                    ) {
                        StepEvent::Start {
                            kind,
                            forward_parent,
                        } => {
                            fp = forward_parent;
                            forward_parent_kinds.push(kind);
                        }
                        StepEvent::Tombstone => (),
                        _ => unreachable!(),
                    }
                }
                for kind in forward_parent_kinds.drain(..).rev() {
                    steps.push(Step::Start { kind });
                }
            }
            StepEvent::Finish => steps.push(Step::Finish),
            StepEvent::Token { kind } => steps.push(Step::Token { kind }),
            StepEvent::Error { message } => steps.push(Step::Error { message }),
            StepEvent::Tombstone => (),
        }
    }

    Output { steps }
}
