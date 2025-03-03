use std::ops::{Index, IndexMut};

use itertools::Itertools;

use crate::ij::{IJSize, IJ};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Mat<T> {
    data: Vec<T>,
    c: IJSize,
}

impl<T> Mat<T> {
    pub fn new(c: IJSize, data: Vec<T>) -> Self {
        assert_eq!(c.size(), data.len());
        Self { data, c }
    }

    pub fn filled(c: IJSize, default: T) -> Self
    where
        T: Clone,
    {
        Self {
            data: vec![default; c.size()],
            c,
        }
    }

    pub fn from_vec(grid: Vec<Vec<T>>) -> Self {
        let h = grid.len();
        // grid が空の場合は幅も高さも 0 としておく
        let w = grid.first().map(|row| row.len()).unwrap_or(0);
        let c = IJSize::new(h, w);
        Mat::new(c, grid.into_iter().flatten().collect_vec())
    }

    pub fn config(&self) -> IJSize {
        self.c
    }

    pub fn get(&self, idx: IJ) -> &T {
        &self.data[idx.index()]
    }

    pub fn get_mut(&mut self, idx: IJ) -> &mut T {
        &mut self.data[idx.index()]
    }

    pub fn row(&self, i: usize) -> &[T] {
        let start = i * self.c.w;
        let end = start + self.c.w;
        &self.data[start..end]
    }

    pub fn row_mut(&mut self, i: usize) -> &mut [T] {
        let start = i * self.c.w;
        let end = start + self.c.w;
        &mut self.data[start..end]
    }
}

impl<T> Index<IJ> for Mat<T> {
    type Output = T;

    fn index(&self, idx: IJ) -> &Self::Output {
        self.get(idx)
    }
}

impl<T> IndexMut<IJ> for Mat<T> {
    fn index_mut(&mut self, idx: IJ) -> &mut Self::Output {
        self.get_mut(idx)
    }
}

impl<T> Index<usize> for Mat<T> {
    type Output = [T];

    fn index(&self, idx: usize) -> &Self::Output {
        self.row(idx)
    }
}

impl<T> IndexMut<usize> for Mat<T> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        self.row_mut(idx)
    }
}

impl<'a, T> IntoIterator for &'a Mat<T> {
    type Item = &'a [T];
    type IntoIter = std::slice::Chunks<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.chunks(self.c.w)
    }
}

impl<'a, T> IntoIterator for &'a mut Mat<T> {
    type Item = &'a mut [T];
    type IntoIter = std::slice::ChunksMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.chunks_mut(self.c.w)
    }
}

impl<T> Mat<T> {
    pub fn iter(&self) -> std::slice::Chunks<'_, T> {
        self.into_iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::ChunksMut<'_, T> {
        self.into_iter()
    }
}

#[macro_export]
macro_rules! mat {
    () => {
        Mat::new(0, 0, vec![])
    };
    ($($($e:expr),*;)*) => {
        {
            let data = vec![$($($e),*),*];
            let h: usize = { 0 $( + { $(let _ = $e;)* 1 })* };
            let w: usize = data.len() / h;
            if data.len() != h * w {
                panic!("invalid matrix size: {} x {} but data len is {}", h, w, data.len());
            }

            Mat::new(IJSize::new(h, w), data)
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mat() {
        let m = mat![
            1, 2, 3;
            4, 5, 6;
            7, 8, 9;
        ];

        let c = m.config();
        assert_eq!(m[c.make(0, 0).unwrap()], 1);
        assert_eq!(m[c.make(0, 1).unwrap()], 2);
        assert_eq!(m[c.make(0, 2).unwrap()], 3);
        assert_eq!(m[c.make(1, 0).unwrap()], 4);
        assert_eq!(m[c.make(1, 1).unwrap()], 5);
        assert_eq!(m[c.make(1, 2).unwrap()], 6);
        assert_eq!(m[c.make(2, 0).unwrap()], 7);
        assert_eq!(m[c.make(2, 1).unwrap()], 8);
        assert_eq!(m[c.make(2, 2).unwrap()], 9);
    }

    #[test]
    fn test_mat_index_twice() {
        let m = mat![
            1, 2, 3;
            4, 5, 6;
            7, 8, 9;
        ];

        assert_eq!(m[0][0], 1);
        assert_eq!(m[0][1], 2);
        assert_eq!(m[0][2], 3);
        assert_eq!(m[1][0], 4);
        assert_eq!(m[1][1], 5);
        assert_eq!(m[1][2], 6);
        assert_eq!(m[2][0], 7);
        assert_eq!(m[2][1], 8);
        assert_eq!(m[2][2], 9);
    }
}
