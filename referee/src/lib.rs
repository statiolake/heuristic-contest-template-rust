use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Score {
    score: f64,
}

pub trait Referee {
    fn run(&self, seed: &str, solution_name: &str) -> Score;
}
