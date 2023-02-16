use crate::input::Input;
use rand::{rngs::SmallRng, thread_rng, SeedableRng};
use std::{fmt, time::Instant};

#[derive(Debug, Clone)]
pub struct Action;

impl fmt::Display for Action {
    fn fmt(&self, b: &mut fmt::Formatter) -> fmt::Result {
        write!(b, "")
    }
}

#[derive(Debug, Clone)]
pub struct Brain {
    gen: SmallRng,
    timer: Instant,
}

impl Brain {
    pub fn init(_input: Input) -> Self {
        let gen = SmallRng::from_rng(thread_rng()).unwrap();
        let timer = Instant::now();

        Self { gen, timer }
    }

    pub fn think(&mut self) -> Action {
        todo!()
    }
}
