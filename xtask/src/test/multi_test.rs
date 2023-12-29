use anyhow::Result;

use referee::MultiReferee;
use solutions::get_solution_names;

pub struct MultiTestConfig<R> {
    referee: R,
    solution_names: Vec<String>,
    players_per_game: usize,
    num_trial_per_solution: usize,
}

impl<R: MultiReferee> MultiTestConfig<R> {
    pub fn new(referee: R) -> Self {
        Self {
            referee,
            solution_names: get_solution_names()
                .into_iter()
                .map(|s| s.to_string())
                .collect(),
            players_per_game: 2,
            num_trial_per_solution: 10,
        }
    }

    pub fn solution_names(mut self, solution_names: Vec<String>) -> Self {
        self.solution_names = solution_names;
        self
    }

    pub fn num_trial_per_solution(mut self, num_trial_per_solution: usize) -> Self {
        self.num_trial_per_solution = num_trial_per_solution;
        self
    }
}

impl<R: MultiReferee> MultiTestConfig<R> {
    pub fn run(self) -> Result<()> {
        self.referee.run(solution_names);

        Ok(())
    }
}
