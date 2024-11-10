use std::{
    fmt,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not},
};

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Bits {
    pub bits: u64,
}

impl fmt::Debug for Bits {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:064b}", self.bits)
    }
}

impl Bits {
    pub fn new() -> Self {
        Self { bits: 0 }
    }

    pub fn set(&mut self, i: usize, b: bool) {
        if b {
            self.bits |= 1 << i;
        } else {
            self.bits &= !(1 << i);
        }
    }

    pub fn get(&self, i: usize) -> bool {
        (self.bits >> i) & 1 == 1
    }

    pub fn count_ones(&self) -> u32 {
        self.bits.count_ones()
    }
}

impl Default for Bits {
    fn default() -> Self {
        Self::new()
    }
}

impl BitAndAssign<Bits> for Bits {
    fn bitand_assign(&mut self, other: Bits) {
        self.bits &= other.bits;
    }
}
impl BitAnd<Bits> for Bits {
    type Output = Self;
    fn bitand(mut self, other: Bits) -> Self::Output {
        self &= other;
        self
    }
}

impl BitOrAssign<Bits> for Bits {
    fn bitor_assign(&mut self, other: Bits) {
        self.bits |= other.bits;
    }
}

impl BitOr<Bits> for Bits {
    type Output = Self;
    fn bitor(mut self, other: Bits) -> Self::Output {
        self |= other;
        self
    }
}

impl Not for Bits {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self { bits: !self.bits }
    }
}
