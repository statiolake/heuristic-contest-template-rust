use crate::Solution;
use io::{InitInput, Output, TurnInput};
use rand::{rngs::SmallRng, thread_rng, SeedableRng};
use std::{ops::ControlFlow, time::Instant};

#[derive(Debug, Clone)]
pub struct NaiveSolution {
    _gen: SmallRng,
    _timer: Instant,
}

impl Solution for NaiveSolution {
    fn name() -> &'static str {
        "naive"
    }

    fn init(_input: InitInput) -> Self {
        let gen = SmallRng::from_rng(thread_rng()).unwrap();
        let timer = Instant::now();

        Self {
            _gen: gen,
            _timer: timer,
        }
    }

    fn think(&mut self, _turn: TurnInput) -> ControlFlow<Output, Output> {
        // TODO: implement this
        todo!()
    }
}
