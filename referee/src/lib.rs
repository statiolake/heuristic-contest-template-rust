use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct MultiScore {
    scores: Vec<f64>,
}

pub trait MultiReferee {
    type Seed;
    const PLAYERS_PER_GAME: usize;

    fn run(&self, seed: Self::Seed, solution_names: [&str; Self::PLAYERS_PER_GAME]) -> MultiScore;
}
