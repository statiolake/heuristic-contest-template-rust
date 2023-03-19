use io::{InitInput, Output, TurnInput};
use rand::{rngs::SmallRng, thread_rng, SeedableRng};
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct Brain {
    gen: SmallRng,
    timer: Instant,
}

impl Brain {
    pub fn init(_input: InitInput) -> Self {
        let gen = SmallRng::from_rng(thread_rng()).unwrap();
        let timer = Instant::now();

        Self { gen, timer }
    }

    pub fn think(&mut self, _turn: TurnInput) -> Output {
        todo!()
    }
}
