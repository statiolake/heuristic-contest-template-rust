use std::fmt;

use anyhow::Result;

struct TestCaseDescription {}

impl fmt::Display for TestCaseDescription {
    fn fmt(&self, b: &mut fmt::Formatter) -> fmt::Result {
        write!(b, "N = (TODO), M = (TODO)")
    }
}

struct RunResult {
    seed: i64,
    desc: TestCaseDescription,
    score: i64,
}

impl fmt::Display for RunResult {
    fn fmt(&self, b: &mut fmt::Formatter) -> fmt::Result {
        write!(
            b,
            "{} ({}) ... score = {}",
            self.seed, self.desc, self.score
        )
    }
}

pub fn main(_args: &[String]) -> Result<()> {}
