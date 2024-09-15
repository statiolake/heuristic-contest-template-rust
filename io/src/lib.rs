extern crate types;

use std::{
    fmt,
    io::BufRead,
    sync::{Mutex, OnceLock},
};

use itertools::{izip, Itertools};
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
pub struct Output {
    pub operations: Vec<Operation>,
}

#[derive(Debug, Clone)]
pub struct Operation {}

impl Operation {}

impl fmt::Display for Output {
    fn fmt(&self, b: &mut fmt::Formatter) -> fmt::Result {
        writeln!(b, "{}", self.operations.len())?;
        for op in &self.operations {
            writeln!(b, "{}", op)?;
        }

        Ok(())
    }
}

impl fmt::Display for Operation {
    fn fmt(&self, _b: &mut fmt::Formatter) -> fmt::Result {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "local", derive(serde::Serialize, serde::Deserialize))]
pub struct InitInput {
    #[cfg_attr(feature = "local", serde(skip))]
    _example: (),
}

impl InitInput {
    pub fn read() -> InitInput {
        let mut locked_stdin = STDIN_SOURCE
            .get_or_init(|| std::sync::Mutex::new(source::Source::new_stdin()))
            .lock()
            .unwrap();
        Self::read_from(&mut *locked_stdin)
    }

    pub fn read_from<R: BufRead>(source: &mut Source<R>) -> InitInput {
        input! {
            from source,
        }

        todo!()
    }

    pub fn description_keys() -> Vec<&'static str> {
        vec![]
    }

    pub fn description_values(&self) -> Vec<String> {
        vec![]
    }

    pub fn describe(&self) -> String {
        izip!(Self::description_keys(), self.description_values(),)
            .map(|(key, value)| format!("{key} = {value}"))
            .join(", ")
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
