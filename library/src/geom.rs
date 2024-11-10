use std::{
    fmt,
    hash::Hash,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

/// 数値型用のトレイト
pub trait Num:
    Copy
    + Clone
    + fmt::Debug
    + PartialEq
    + PartialOrd
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
    + Neg<Output = Self>
{
}

/// 整数用の追加トレイト
pub trait Integer: Num + Eq + Ord + Hash {}

/// 浮動小数点数用の追加トレイト
pub trait Float: Num {
    fn sqrt(self) -> Self;
    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn acos(self) -> Self;
    fn abs(self) -> Self;
}

// 基本的な数値型に対してNumトレイトを実装
macro_rules! impl_num {
    ($($t:ty),*) => {
        $(impl Num for $t {})*
    };
}

impl_num!(i8, i16, i32, i64, isize, f32, f64);

macro_rules! impl_float {
    ($($t:ty),*) => {
        $(impl Float for $t {
            fn sqrt(self) -> Self {
                self.sqrt()
            }
            fn sin(self) -> Self {
                self.sin()
            }
            fn cos(self) -> Self {
                self.cos()
            }
            fn acos(self) -> Self {
                self.acos()
            }
            fn abs(self) -> Self {
                self.abs()
            }
        })*
    };
}

impl_float!(f32, f64);

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

// 浮動小数点数型でのみ利用可能な実装
impl<T: Float> Vec2D<T> {
    pub fn length_squared(&self) -> T {
        self.x * self.x + self.y * self.y
    }

    pub fn length(&self) -> T {
        self.length_squared().sqrt()
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
