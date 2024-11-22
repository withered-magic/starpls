use drop_bomb::DropBomb;

use crate::step::StepEvent;
use crate::Parser;
use crate::SyntaxKind;

pub(crate) struct Marker {
    pos: u32,
    bomb: DropBomb,
}

impl Marker {
    pub(crate) fn new(pos: u32) -> Marker {
        Marker {
            pos,
            bomb: DropBomb::new("marker must either be completed or abandoned"),
        }
    }

    pub(crate) fn complete(mut self, p: &mut Parser, kind: SyntaxKind) -> CompletedMarker {
        self.bomb.defuse();
        p.events[self.pos as usize] = StepEvent::Start {
            kind,
            forward_parent: None,
        };
        p.push_event(StepEvent::Finish);
        CompletedMarker::new(self.pos)
    }

    pub(crate) fn abandon(mut self, p: &mut Parser) {
        self.bomb.defuse();

        // Optimization: If this marker corresponds to the most recent event, we can actually
        // get rid of it altogether, saving us some space.
        if self.pos as usize == p.events.len() - 1 {
            match p.events.pop() {
                Some(StepEvent::Tombstone) => (),
                _ => unreachable!(),
            }
        }
    }
}

pub(crate) struct CompletedMarker {
    pos: u32,
}

impl CompletedMarker {
    pub(crate) fn new(pos: u32) -> Self {
        Self { pos }
    }

    pub(crate) fn precede(&mut self, p: &mut Parser) -> Marker {
        let m = p.start();
        match &mut p.events[self.pos as usize] {
            StepEvent::Start { forward_parent, .. } => *forward_parent = Some(m.pos),
            _ => unreachable!(),
        }
        m
    }
}
