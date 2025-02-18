use std::{
    fmt, mem,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not},
};

use itertools::{izip, Itertools};

use crate::{bits::Bits, geom::Vec2D, matrix::Mat};
use std::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Grid<T> {
    pub n: usize,
    pub mat: Mat<T>,
}

impl<T: Default + Clone> Grid<T> {
    pub fn new_default(n: usize) -> Self {
        let mat = Mat::filled(n, n, T::default());
        Self { n, mat }
    }
}

impl<T> Grid<T> {
    pub fn from_vec(grid: Vec<Vec<T>>) -> Self {
        let n = grid.len();
        assert!(grid.iter().all(|row| row.len() == n), "Grid must be square");
        let mat = Mat::new(n, n, grid.into_iter().flatten().collect_vec());

        Self { n, mat }
    }

    pub fn compute_points(&self) -> Vec<Vec2D<i32>>
    where
        T: PartialEq + Default,
    {
        let mut points = vec![];

        for i in 0..self.n as i32 {
            for j in 0..self.n as i32 {
                let pos = Vec2D::new(j, i);
                if self.get(pos).map_or(false, |v| v != &T::default()) {
                    points.push(pos);
                }
            }
        }

        points
    }

    pub fn get(&self, p: Vec2D<i32>) -> Option<&T> {
        if p.y < 0 || p.y >= self.n as i32 || p.x < 0 || p.x >= self.n as i32 {
            None
        } else {
            let i = p.y as usize;
            let j = p.x as usize;
            Some(&self.mat[i][j])
        }
    }

    pub fn get_mut(&mut self, p: Vec2D<i32>) -> Option<&mut T> {
        if p.y < 0 || p.y >= self.n as i32 || p.x < 0 || p.x >= self.n as i32 {
            None
        } else {
            let i = p.y as usize;
            let j = p.x as usize;
            Some(&mut self.mat[i][j])
        }
    }

    pub fn set(&mut self, p: Vec2D<i32>, value: T) -> bool {
        if let Some(cell) = self.get_mut(p) {
            *cell = value;
            true
        } else {
            false
        }
    }

    pub fn dump(&self)
    where
        T: fmt::Display,
    {
        eprintln!("+{}+", "-".repeat(self.n));
        for i in 0..self.n as i32 {
            eprint!("|");
            for j in 0..self.n as i32 {
                let pos = Vec2D::new(j, i);
                eprint!("{}", self.get(pos).unwrap());
            }
            eprintln!("|");
        }
        eprintln!("+{}+", "-".repeat(self.n));
    }
}

// ビット演算のトレイト実装（bool型に特化）
impl BitAndAssign for Grid<bool> {
    fn bitand_assign(&mut self, other: Self) {
        assert_eq!(self.n, other.n);
        for i in 0..self.n {
            for j in 0..self.n {
                self.mat[i][j] &= other.mat[i][j];
            }
        }
    }
}

impl BitAnd for Grid<bool> {
    type Output = Self;

    fn bitand(mut self, other: Self) -> Self::Output {
        self &= other;
        self
    }
}

impl BitOrAssign for Grid<bool> {
    fn bitor_assign(&mut self, other: Self) {
        assert_eq!(self.n, other.n);
        for i in 0..self.n {
            for j in 0..self.n {
                self.mat[i][j] |= other.mat[i][j];
            }
        }
    }
}

impl BitOr for Grid<bool> {
    type Output = Self;

    fn bitor(mut self, other: Self) -> Self::Output {
        self |= other;
        self
    }
}

impl Not for Grid<bool> {
    type Output = Self;

    fn not(mut self) -> Self::Output {
        for row in &mut self.mat {
            for cell in row {
                *cell = !*cell;
            }
        }
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BitField {
    pub n: usize,
    pub rows: [Bits; 64],
}

impl BitField {
    pub fn new_zero(n: usize) -> Self {
        assert!(n < 64);
        let rows = [Bits::new(); 64];

        Self { n, rows }
    }

    pub fn from_grid(grid: Grid<bool>) -> Self {
        let n = grid.n;
        assert!(n < 64);

        let mut rows = [Bits::new(); 64];

        for (i, row) in grid.mat.iter().enumerate() {
            assert_eq!(row.len(), n);
            let mut bits = Bits::new();

            for (j, &b) in row.iter().enumerate() {
                bits.set(j, b);
            }

            rows[i] = bits;
        }

        Self { n, rows }
    }

    pub fn compute_points(&self) -> Vec<Vec2D<i32>> {
        let mut points = vec![];

        for i in 0..self.n as i32 {
            for j in 0..self.n as i32 {
                let pos = Vec2D::new(j, i);
                if self.get(pos).unwrap() {
                    points.push(pos);
                }
            }
        }

        points
    }

    pub fn get(&self, p: Vec2D<i32>) -> Option<bool> {
        if p.y < 0 || p.y >= self.n as i32 || p.x < 0 || p.x >= self.n as i32 {
            None
        } else {
            let i = p.y as usize;
            let j = p.x as usize;
            Some(self.rows[i].get(j))
        }
    }

    pub fn set(&mut self, p: Vec2D<i32>, b: bool) -> bool {
        if p.y < 0 || p.y >= self.n as i32 || p.x < 0 || p.x >= self.n as i32 {
            false
        } else {
            let i = p.y as usize;
            let j = p.x as usize;
            self.rows[i].set(j, b);

            true
        }
    }

    pub fn dump(&self) {
        eprintln!("+{}+", "-".repeat(self.n));
        for i in 0..self.n as i32 {
            eprint!("|");
            for j in 0..self.n as i32 {
                let pos = Vec2D::new(j, i);
                let ch = if self.get(pos).unwrap() { '#' } else { '.' };
                eprint!("{ch}");
            }
            eprintln!("|");
        }
        eprintln!("+{}+", "-".repeat(self.n));
    }

    pub fn count_ones(&self) -> u32 {
        self.rows.iter().map(|r| r.count_ones()).sum()
    }
}

impl BitAndAssign<BitField> for BitField {
    fn bitand_assign(&mut self, other: BitField) {
        assert_eq!(self.n, other.n);
        for (a, b) in izip!(&mut self.rows, &other.rows) {
            *a &= *b;
        }
    }
}

impl BitAnd<BitField> for BitField {
    type Output = Self;
    fn bitand(mut self, other: BitField) -> Self::Output {
        self &= other;
        self
    }
}

impl BitOrAssign<BitField> for BitField {
    fn bitor_assign(&mut self, other: BitField) {
        assert_eq!(self.n, other.n);
        for (a, b) in izip!(&mut self.rows, &other.rows) {
            *a |= *b;
        }
    }
}

impl BitOr<BitField> for BitField {
    type Output = Self;
    fn bitor(mut self, other: BitField) -> Self::Output {
        self |= other;
        self
    }
}

impl Not for BitField {
    type Output = Self;

    fn not(self) -> Self::Output {
        let mut rows = self.rows;
        rows.iter_mut().for_each(|r| *r = !*r);

        Self { n: self.n, rows }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Direction {
    U = 0,
    R = 1,
    D = 2,
    L = 3,
}

impl Direction {
    pub fn all() -> [Direction; 4] {
        [Direction::U, Direction::R, Direction::D, Direction::L]
    }

    pub fn to_vector(&self) -> Vec2D<i32> {
        match self {
            Direction::U => Vec2D::new(0, -1),
            Direction::R => Vec2D::new(1, 0),
            Direction::D => Vec2D::new(0, 1),
            Direction::L => Vec2D::new(-1, 0),
        }
    }

    pub fn is_opposite_to(&self, other: Direction) -> bool {
        matches!(
            (self, other),
            (Direction::U, Direction::D)
                | (Direction::R, Direction::L)
                | (Direction::D, Direction::U)
                | (Direction::L, Direction::R)
        )
    }

    pub fn rotate(self, r: Rotate) -> Self {
        // SAFETY: Direction は 0, 1, 2, 3 なので 4 で割った余りは必ず有効な variant になる
        unsafe { mem::transmute((self as u8 + r as u8) & 0b11) }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, b: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Direction::U => write!(b, "U"),
            Direction::R => write!(b, "R"),
            Direction::D => write!(b, "D"),
            Direction::L => write!(b, "L"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rotate {
    /// 何もしない
    S = 0,

    /// 反時計回り
    L = 3,

    /// 時計回り
    R = 1,
}

impl Rotate {
    pub fn all() -> [Rotate; 3] {
        [Rotate::S, Rotate::L, Rotate::R]
    }
}
