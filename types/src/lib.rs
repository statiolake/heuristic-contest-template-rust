extern crate io;

use std::ops::ControlFlow;

use io::{InitInput, Output, TurnInput};

pub trait Solution {
    fn name() -> &'static str
    where
        Self: Sized;

    fn init(input: InitInput) -> Self
    where
        Self: Sized;

    fn think(&mut self, turn: TurnInput) -> ControlFlow<Output, Output>;
}
