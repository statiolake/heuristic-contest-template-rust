use std::{
    fmt,
    sync::{Mutex, OnceLock},
};

use source::Source;

pub mod io;
pub mod macros;
pub mod source;

pub static STDIN_SOURCE: OnceLock<Mutex<Source>> = OnceLock::new();

// Hack: You need this wrapper to "surpress" path conversion in bundler. Without this wrapper, you
// need to call `input!` macro like `crate::input!`. This is transformed to `crate::io::input!`,
// which is not collect.
macro_rules! input {
    ($($tokens:tt)*) => {
        $crate::input!($($tokens)*)
    };
}

#[derive(Debug, Clone)]
pub struct Output;

impl fmt::Display for Output {
    fn fmt(&self, _b: &mut fmt::Formatter) -> fmt::Result {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InitInput {}

impl InitInput {
    pub fn read() -> InitInput {
        input! {}

        InitInput {}
    }

    pub fn describe(&self) -> String {
        "TODO: insert description here".into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TurnInput {}

impl TurnInput {
    pub fn read() -> TurnInput {
        input! {}

        TurnInput {}
    }
}
