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
