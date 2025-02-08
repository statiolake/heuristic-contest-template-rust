extern crate referee;

use std::{
    io::{BufRead, Write},
    sync::{Mutex, OnceLock},
};

use referee::{InitInput, Output, TurnInput};
use source::Source;
use traits::{ReadInput, WriteOutput};

pub mod io;
pub mod macros;
pub mod source;
pub mod traits;

pub static STDIN_SOURCE: OnceLock<Mutex<Source>> = OnceLock::new();

// Hack: You need this wrapper to "surpress" path conversion in bundler. Without this wrapper, you
// need to call `input!` macro like `crate::input!`. This is transformed to `crate::io::input!`,
// which is not collect.
macro_rules! input {
    ($($tokens:tt)*) => {
        $crate::input!($($tokens)*)
    };
}

// Implementation for each inputs

impl ReadInput for InitInput {
    fn read_from<R: BufRead>(source: &mut Source<R>) -> InitInput {
        input! {
            from source,
        }

        InitInput {}
    }
}

impl ReadInput for TurnInput {
    fn read_from<R: BufRead>(source: &mut Source<R>) -> Self {
        input! {
            from source,
        }

        TurnInput {}
    }
}

impl WriteOutput for Output {
    fn write_to<W: Write>(&self, b: &mut W) {
        writeln!(b, "{}", self.operations.len()).unwrap();
        for operation in &self.operations {
            writeln!(b, "{}", operation).unwrap();
        }
    }
}
