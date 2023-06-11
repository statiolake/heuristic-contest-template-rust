extern crate io;
extern crate types;

use io::{Action, InitInput, TurnInput};
use rand::{rngs::SmallRng, thread_rng, SeedableRng};
use std::time::Instant;
use types::Solution;

#[derive(Debug, Clone)]
pub struct FirstSolution {
    gen: SmallRng,
    timer: Instant,
}

impl Solution for FirstSolution {
    fn init(_input: InitInput) -> Self {
        let gen = SmallRng::from_rng(thread_rng()).unwrap();
        let timer = Instant::now();

        Self { gen, timer }
    }

    fn think(&mut self, _turn: TurnInput) -> Action {
        // TODO: implement this
        todo!()
    }
}
