use self::SyntaxKind::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(non_camel_case_types)]
#[repr(u16)]
pub enum SyntaxKind {
    // Tokens.
    ERROR,
    EOF,

    MODULE,
}

impl From<u16> for SyntaxKind {
    #[inline]
    fn from(value: u16) -> Self {
        assert!(value <= MODULE as u16);
        unsafe { std::mem::transmute(value) }
    }
}

impl From<SyntaxKind> for u16 {
    #[inline]
    fn from(kind: SyntaxKind) -> Self {
        kind as u16
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SyntaxKindSet(u128);

impl SyntaxKindSet {
    pub const fn new(kinds: &[SyntaxKind]) -> SyntaxKindSet {
        let mut inner = 0;
        let mut i = 0;
        while i < kinds.len() {
            inner |= 1 << kinds[i] as u16;
            i += 1;
        }
        SyntaxKindSet(inner)
    }

    pub const fn contains(&self, kind: SyntaxKind) -> bool {
        self.0 & 1 << kind as usize > 0
    }

    pub const fn union(&self, other: SyntaxKindSet) -> SyntaxKindSet {
        SyntaxKindSet(self.0 | other.0)
    }
}
