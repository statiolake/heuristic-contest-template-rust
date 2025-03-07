use std::{
    fmt,
    hash::Hash,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use crate::num::{Float, Num};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vec2D<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2D<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Copy> Vec2D<T> {
    pub fn x(&self) -> T {
        self.x
    }

    pub fn y(&self) -> T {
        self.y
    }

    pub fn cast<U>(&self) -> Vec2D<U>
    where
        U: TryFrom<T>,
        U::Error: fmt::Debug,
    {
        Vec2D::new(self.x.try_into().unwrap(), self.y.try_into().unwrap())
    }
}

impl<T: Num> Vec2D<T> {
    // マンハッタン距離
    pub fn manhattan_distance(&self, other: &Self) -> T {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        dx + dy
    }

    pub fn square_distance(&self, other: &Self) -> T {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        dx * dx + dy * dy
    }
}

impl<T: Float> Vec2D<T> {
    pub fn eucrid_distance(&self, other: &Self) -> T {
        self.square_distance(other).sqrt()
    }
}

// 浮動小数点数型でのみ利用可能な実装
impl<T: Float> Vec2D<T> {
    pub fn square_length(&self) -> T {
        self.x * self.x + self.y * self.y
    }

    pub fn length(&self) -> T {
        self.square_length().sqrt()
    }

    pub fn normalize(&self) -> Self {
        let len = self.length();
        Self {
            x: self.x / len,
            y: self.y / len,
        }
    }

    pub fn dot(&self, other: &Self) -> T {
        self.x * other.x + self.y * other.y
    }

    pub fn cross(&self, other: &Self) -> T {
        self.x * other.y - self.y * other.x
    }

    pub fn angle_to(&self, other: &Self) -> T {
        (self.dot(other) / (self.length() * other.length())).acos()
    }

    pub fn rotate(&self, angle: T) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        Self {
            x: self.x * cos - self.y * sin,
            y: self.x * sin + self.y * cos,
        }
    }

    pub fn distance_to(&self, other: &Self) -> T {
        (*other - *self).length()
    }

    pub fn approx_eq(&self, other: &Self, eps: T) -> bool {
        (self.x - other.x).abs() < eps && (self.y - other.y).abs() < eps
    }
}

// 演算子のオーバーロード実装
impl<T: Num> Add for Vec2D<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Num> Sub for Vec2D<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T: Num> Mul<T> for Vec2D<T> {
    type Output = Self;
    fn mul(self, scalar: T) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl<T: Num> Div<T> for Vec2D<T> {
    type Output = Self;
    fn div(self, scalar: T) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}

impl<T: Num> Neg for Vec2D<T> {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<T: Num> AddAssign for Vec2D<T> {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl<T: Num> SubAssign for Vec2D<T> {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl<T: Num> MulAssign<T> for Vec2D<T> {
    fn mul_assign(&mut self, scalar: T) {
        *self = *self * scalar;
    }
}

impl<T: Num> DivAssign<T> for Vec2D<T> {
    fn div_assign(&mut self, scalar: T) {
        *self = *self / scalar;
    }
}
