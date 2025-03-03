use crate::{
    ij::{IJSize, IJ},
    matrix::Mat,
};
use std::{fmt, vec::Vec};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Grid<T> {
    pub mat: Mat<T>,
}

impl<T: Default + Clone> Grid<T> {
    pub fn new_default(c: IJSize) -> Self {
        let mat = Mat::filled(c, T::default());
        Self { mat }
    }
}

impl<T> Grid<T> {
    pub fn from_vec(grid: Vec<Vec<T>>) -> Self {
        let mat = Mat::from_vec(grid);

        Self { mat }
    }

    pub fn config(&self) -> IJSize {
        self.mat.config()
    }

    pub fn compute_points(&self) -> Vec<IJ>
    where
        T: PartialEq + Default,
    {
        let mut points = vec![];

        for i in 0..self.config().h {
            for j in 0..self.config().w {
                let pos = unsafe { self.config().make_unchecked(i, j) };
                if self.get(pos) != &T::default() {
                    points.push(pos);
                }
            }
        }

        points
    }

    pub fn get(&self, p: IJ) -> &T {
        self.mat.get(p)
    }

    pub fn get_mut(&mut self, p: IJ) -> &mut T {
        self.mat.get_mut(p)
    }

    pub fn set(&mut self, p: IJ, value: T) {
        *self.get_mut(p) = value;
    }

    pub fn dump(&self)
    where
        T: fmt::Display,
    {
        eprintln!("+{}+", "-".repeat(self.config().w));
        for i in 0..self.config().h {
            eprint!("|");
            for j in 0..self.config().w {
                let pos = unsafe { self.config().make_unchecked(i, j) };
                eprint!("{}", self.get(pos));
            }
            eprintln!("|");
        }
        eprintln!("+{}+", "-".repeat(self.config().w));
    }
}
