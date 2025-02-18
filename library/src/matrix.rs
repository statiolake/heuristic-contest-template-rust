use std::ops::{Index, IndexMut};

use crate::geom::Vec2D;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Mat<T> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T> Mat<T> {
    pub fn new(rows: usize, cols: usize, data: Vec<T>) -> Self {
        assert_eq!(rows * cols, data.len());
        Self { data, rows, cols }
    }

    pub fn filled(rows: usize, cols: usize, default: T) -> Self
    where
        T: Clone,
    {
        Self {
            data: vec![default; rows * cols],
            rows,
            cols,
        }
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn get(&self, idx: Vec2D<usize>) -> &T {
        &self.data[idx.y() * self.cols + idx.x()]
    }

    pub fn get_mut(&mut self, idx: Vec2D<usize>) -> &mut T {
        &mut self.data[idx.y() * self.cols + idx.x()]
    }

    pub fn row(&self, i: usize) -> &[T] {
        &self.data[i * self.cols..(i + 1) * self.cols]
    }

    pub fn row_mut(&mut self, i: usize) -> &mut [T] {
        &mut self.data[i * self.cols..(i + 1) * self.cols]
    }
}

impl<T> Index<Vec2D<usize>> for Mat<T> {
    type Output = T;

    fn index(&self, idx: Vec2D<usize>) -> &Self::Output {
        self.get(idx)
    }
}

impl<T> IndexMut<Vec2D<usize>> for Mat<T> {
    fn index_mut(&mut self, idx: Vec2D<usize>) -> &mut Self::Output {
        self.get_mut(idx)
    }
}

impl<T> Index<(usize, usize)> for Mat<T> {
    type Output = T;

    fn index(&self, idx: (usize, usize)) -> &Self::Output {
        self.get(Vec2D::new(idx.1, idx.0))
    }
}

impl<T> IndexMut<(usize, usize)> for Mat<T> {
    fn index_mut(&mut self, idx: (usize, usize)) -> &mut Self::Output {
        self.get_mut(Vec2D::new(idx.1, idx.0))
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
        self.data.chunks(self.cols)
    }
}

impl<'a, T> IntoIterator for &'a mut Mat<T> {
    type Item = &'a mut [T];
    type IntoIter = std::slice::ChunksMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.chunks_mut(self.cols)
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
            let rows: usize = { 0 $( + { $(let _ = $e;)* 1 })* };
            let cols: usize = data.len() / rows;
            if data.len() != rows * cols {
                panic!("invalid matrix size: {} x {} but data len is {}", rows, cols, data.len());
            }

            Mat::new(rows, cols, data)
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

        assert_eq!(m[(0, 0)], 1);
        assert_eq!(m[(0, 1)], 2);
        assert_eq!(m[(0, 2)], 3);
        assert_eq!(m[(1, 0)], 4);
        assert_eq!(m[(1, 1)], 5);
        assert_eq!(m[(1, 2)], 6);
        assert_eq!(m[(2, 0)], 7);
        assert_eq!(m[(2, 1)], 8);
        assert_eq!(m[(2, 2)], 9);
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
