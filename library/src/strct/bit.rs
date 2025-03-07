use std::{
    fmt,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not},
};

use itertools::izip;

use crate::strct::{
    grid::Grid,
    ij::{IJSize, IJ},
};

type BitRowRepr = u64;

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BitRow {
    pub bits: BitRowRepr,
}

impl fmt::Debug for BitRow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // BitsRepr の型が変わったときにフォーマット指定子を変え忘れないようにするため、あえてここで
        // u64 に代入して型が変わっていないことを静的にチェックしている
        let bits: u64 = self.bits;
        write!(f, "{:064b}", bits)
    }
}

impl BitRow {
    pub const NUM_BITS: usize = BitRowRepr::BITS as usize;

    pub fn new() -> Self {
        Self { bits: 0 }
    }

    pub fn set(&mut self, i: usize, b: bool) {
        self.bits = (self.bits & !(1 << i)) | ((b as BitRowRepr) << i);
    }

    pub fn get(&self, i: usize) -> bool {
        (self.bits >> i) & 1 == 1
    }

    pub fn count_ones(&self) -> u32 {
        self.bits.count_ones()
    }
}

impl Default for BitRow {
    fn default() -> Self {
        Self::new()
    }
}

impl BitAndAssign<BitRow> for BitRow {
    fn bitand_assign(&mut self, other: BitRow) {
        self.bits &= other.bits;
    }
}

impl BitAnd<BitRow> for BitRow {
    type Output = Self;
    fn bitand(mut self, other: BitRow) -> Self::Output {
        self &= other;
        self
    }
}

impl BitOrAssign<BitRow> for BitRow {
    fn bitor_assign(&mut self, other: BitRow) {
        self.bits |= other.bits;
    }
}

impl BitOr<BitRow> for BitRow {
    type Output = Self;
    fn bitor(mut self, other: BitRow) -> Self::Output {
        self |= other;
        self
    }
}

impl Not for BitRow {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self { bits: !self.bits }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BitGrid {
    pub c: IJSize,
    pub rows: Vec<BitRow>,
}

impl BitGrid {
    pub fn new_zero(c: IJSize) -> Self {
        assert!(c.w < BitRow::NUM_BITS);
        let rows = vec![BitRow::new(); c.h];

        Self { c, rows }
    }

    pub fn from_grid(grid: Grid<bool>) -> Self {
        let c = grid.config();

        let mut rows = vec![BitRow::new(); c.h];

        for (i, row) in grid.iter().enumerate() {
            assert_eq!(row.len(), c.w);
            let mut bits = BitRow::new();

            for (j, &b) in row.iter().enumerate() {
                bits.set(j, b);
            }

            rows[i] = bits;
        }

        Self { c, rows }
    }

    pub fn compute_points(&self) -> Vec<IJ> {
        let mut points = vec![];

        for i in 0..self.c.h {
            for j in 0..self.c.w {
                // SAFETY: ここでは i, j が有効なインデックスであることが保証されている
                let pos = unsafe { IJ::from_pair_unchecked(self.c, i, j) };
                if self.get(pos) {
                    points.push(pos);
                }
            }
        }

        points
    }

    pub fn get(&self, p: IJ) -> bool {
        let (i, j) = p.to_pair(self.c);

        self.rows[i].get(j)
    }

    pub fn set(&mut self, p: IJ, b: bool) {
        let (i, j) = p.to_pair(self.c);
        self.rows[i].set(j, b);
    }

    pub fn dump(&self) {
        eprintln!("+{}+", "-".repeat(self.c.w));
        for i in 0..self.c.h {
            eprint!("|");
            for j in 0..self.c.w {
                // SAFETY: ここでは i, j が有効なインデックスであることが保証されている
                let pos = unsafe { IJ::from_pair_unchecked(self.c, i, j) };
                let ch = if self.get(pos) { '#' } else { '.' };
                eprint!("{ch}");
            }
            eprintln!("|");
        }
        eprintln!("+{}+", "-".repeat(self.c.w));
    }

    pub fn count_ones(&self) -> u32 {
        self.rows.iter().map(|r| r.count_ones()).sum()
    }
}

impl BitAndAssign<BitGrid> for BitGrid {
    fn bitand_assign(&mut self, other: BitGrid) {
        assert_eq!(self.c, other.c);
        for (a, b) in izip!(&mut self.rows, &other.rows) {
            *a &= *b;
        }
    }
}

impl BitAnd<BitGrid> for BitGrid {
    type Output = Self;
    fn bitand(mut self, other: BitGrid) -> Self::Output {
        self &= other;
        self
    }
}

impl BitOrAssign<BitGrid> for BitGrid {
    fn bitor_assign(&mut self, other: BitGrid) {
        assert_eq!(self.c, other.c);
        for (a, b) in izip!(&mut self.rows, &other.rows) {
            *a |= *b;
        }
    }
}

impl BitOr<BitGrid> for BitGrid {
    type Output = Self;
    fn bitor(mut self, other: BitGrid) -> Self::Output {
        self |= other;
        self
    }
}

impl Not for BitGrid {
    type Output = Self;

    fn not(self) -> Self::Output {
        let c = self.c;
        let mut rows = self.rows;
        rows.iter_mut().for_each(|r| *r = !*r);

        Self { c, rows }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let bits = BitRow::new();
        assert_eq!(bits.bits, 0);
    }

    #[test]
    fn test_set_get() {
        let mut bits = BitRow::new();
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
        let mut bits = BitRow::new();
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
        let mut a = BitRow::new();
        let mut b = BitRow::new();
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
        let mut bits = BitRow::new();
        assert_eq!(bits.count_ones(), 0);
        bits.set(1, true);
        bits.set(3, true);
        bits.set(5, true);
        assert_eq!(bits.count_ones(), 3);
    }

    #[test]
    fn test_default() {
        let bits: BitRow = Default::default();
        assert_eq!(bits.bits, 0);
    }
}
