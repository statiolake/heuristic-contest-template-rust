use std::{
    fmt,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not},
};

type BitsRepr = u64;

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Bits {
    pub bits: BitsRepr,
}

impl fmt::Debug for Bits {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:064b}", self.bits)
    }
}

impl Bits {
    pub const NUM_BITS: usize = BitsRepr::BITS as usize;

    pub fn new() -> Self {
        Self { bits: 0 }
    }

    pub fn set(&mut self, i: usize, b: bool) {
        self.bits = (self.bits & !(1 << i)) | ((b as BitsRepr) << i);
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

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let bits = Bits::new();
        assert_eq!(bits.bits, 0);
    }

    #[test]
    fn test_set_get() {
        let mut bits = Bits::new();
        assert!(!bits.get(0));
        assert!(!bits.get(1));
        bits.set(1, true);
        assert!(!bits.get(0));
        assert!(bits.get(1));
        bits.set(1, false);
        assert!(!bits.get(0));
        assert!(!bits.get(1));
    }

    #[test]
    fn test_multiple_bits() {
        let mut bits = Bits::new();
        bits.set(0, true);
        bits.set(2, true);
        bits.set(4, true);
        assert!(bits.get(0));
        assert!(!bits.get(1));
        assert!(bits.get(2));
        assert!(!bits.get(3));
        assert!(bits.get(4));
    }

    #[test]
    fn test_bit_ops() {
        let mut a = Bits::new();
        let mut b = Bits::new();
        a.set(1, true);
        a.set(3, true);
        b.set(3, true);
        b.set(5, true);

        // AND
        let c = a & b;
        assert!(!c.get(1));
        assert!(c.get(3));
        assert!(!c.get(5));

        // OR
        let d = a | b;
        assert!(d.get(1));
        assert!(d.get(3));
        assert!(d.get(5));

        // NOT
        let e = !a;
        assert!(e.get(0));
        assert!(!e.get(1));
        assert!(e.get(2));
        assert!(!e.get(3));
    }

    #[test]
    fn test_count_ones() {
        let mut bits = Bits::new();
        assert_eq!(bits.count_ones(), 0);
        bits.set(1, true);
        bits.set(3, true);
        bits.set(5, true);
        assert_eq!(bits.count_ones(), 3);
    }

    #[test]
    fn test_default() {
        let bits: Bits = Default::default();
        assert_eq!(bits.bits, 0);
    }
}
