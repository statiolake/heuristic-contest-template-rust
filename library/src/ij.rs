use std::{
    fmt, mem,
    ops::{Add, Sub},
};

use crate::{make_per, matrix::Mat};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct IJ(usize);

impl IJ {
    pub fn from_pair(c: IJSize, i: usize, j: usize) -> Option<Self> {
        if i >= c.h || j >= c.w {
            None
        } else {
            Some(Self(i * c.w + j))
        }
    }

    /// # Safety
    ///
    /// `i` と `j` は有効なインデックスであること
    pub unsafe fn from_pair_unchecked(c: IJSize, i: usize, j: usize) -> Self {
        Self(i * c.w + j)
    }

    pub fn to_pair(self, c: IJSize) -> (usize, usize) {
        (self.0 / c.w, self.0 % c.w)
    }

    pub fn index(self) -> usize {
        self.0
    }

    pub fn dijs(c: IJSize) -> PerIJDir<Self> {
        PerIJDir::new([
            Self(0usize.wrapping_sub(c.w)),
            Self(1),
            Self(c.w),
            Self(0usize.wrapping_sub(1)),
        ])
    }

    pub fn generate_neighbors(c: IJSize) -> Mat<PerIJDir<Option<Self>>> {
        let mut res = vec![PerIJDir::default(); c.size()];

        for i in 0..c.h {
            for j in 0..c.w {
                for d in IJDir::all() {
                    // SAFETY: i, j は c.h, c.w より小さいため有効なインデックス
                    let ij = unsafe { c.make_unchecked(i, j) };
                    let Some((ni, nj)) = (match d {
                        IJDir::U => i.checked_sub(1).map(|i| (i, j)),
                        IJDir::R => Some((i, j + 1)).filter(|(_, j)| *j < c.w),
                        IJDir::D => Some((i + 1, j)).filter(|(i, _)| *i < c.h),
                        IJDir::L => j.checked_sub(1).map(|j| (i, j)),
                    }) else {
                        continue;
                    };

                    let nij = unsafe { c.make_unchecked(ni, nj) };

                    res[ij.index()][d] = Some(nij);
                }
            }
        }

        Mat::new(c, res)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct IJSize {
    pub w: usize,
    pub h: usize,
}

impl IJSize {
    pub fn new(h: usize, w: usize) -> Self {
        Self { w, h }
    }

    pub fn size(self) -> usize {
        self.h * self.w
    }

    pub fn make(self, i: usize, j: usize) -> Option<IJ> {
        IJ::from_pair(self, i, j)
    }

    /// # Safety
    ///
    /// `i` と `j` は有効なインデックスであること
    pub unsafe fn make_unchecked(self, i: usize, j: usize) -> IJ {
        IJ::from_pair_unchecked(self, i, j)
    }
}

// 演算子のオーバーロード実装
impl Add for IJ {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl Sub for IJ {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum IJDir {
    U = 0,
    R = 1,
    D = 2,
    L = 3,
}

impl IJDir {
    pub fn all() -> [IJDir; 4] {
        [IJDir::U, IJDir::R, IJDir::D, IJDir::L]
    }

    pub fn is_opposite_to(self, other: IJDir) -> bool {
        // 下 1 ビットが同じなら逆方向になる
        self != other && (self as usize & 1 == other as usize & 1)
    }

    pub fn rotate(self, r: Rotate) -> Self {
        // SAFETY: Direction は 0, 1, 2, 3 なので 4 で割った余りは必ず有効な variant になる
        unsafe { mem::transmute((self as u8 + r as u8) & 0b11) }
    }
}

impl fmt::Display for IJDir {
    fn fmt(&self, b: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IJDir::U => write!(b, "U"),
            IJDir::R => write!(b, "R"),
            IJDir::D => write!(b, "D"),
            IJDir::L => write!(b, "L"),
        }
    }
}

make_per!(PerIJDir, IJDir, 4);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neighbors() {
        let c = IJSize::new(3, 3);
        let neighbors = IJ::generate_neighbors(c);

        // 左上
        assert_eq!(neighbors[c.make(0, 0).unwrap()][IJDir::U], None);
        assert_eq!(
            neighbors[c.make(0, 0).unwrap()][IJDir::R],
            Some(c.make(0, 1).unwrap())
        );
        assert_eq!(
            neighbors[c.make(0, 0).unwrap()][IJDir::D],
            Some(c.make(1, 0).unwrap())
        );
        assert_eq!(neighbors[c.make(0, 0).unwrap()][IJDir::L], None);

        // 右下
        assert_eq!(
            neighbors[c.make(2, 2).unwrap()][IJDir::U],
            Some(c.make(1, 2).unwrap())
        );
        assert_eq!(neighbors[c.make(2, 2).unwrap()][IJDir::R], None);
        assert_eq!(neighbors[c.make(2, 2).unwrap()][IJDir::D], None);
        assert_eq!(
            neighbors[c.make(2, 2).unwrap()][IJDir::L],
            Some(c.make(2, 1).unwrap())
        );

        // 中央
        assert_eq!(
            neighbors[c.make(1, 1).unwrap()][IJDir::U],
            Some(c.make(0, 1).unwrap())
        );
        assert_eq!(
            neighbors[c.make(1, 1).unwrap()][IJDir::R],
            Some(c.make(1, 2).unwrap())
        );
        assert_eq!(
            neighbors[c.make(1, 1).unwrap()][IJDir::D],
            Some(c.make(2, 1).unwrap())
        );
        assert_eq!(
            neighbors[c.make(1, 1).unwrap()][IJDir::L],
            Some(c.make(1, 0).unwrap())
        );
    }
}
