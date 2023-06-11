extern crate io;

use io::{Action, InitInput, TurnInput};

pub trait Solution {
    fn init(input: InitInput) -> Self;
    fn think(&mut self, turn: TurnInput) -> Action;
}
