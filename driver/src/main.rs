extern crate io;
extern crate solution_01;
extern crate types;

use io::{InitInput, TurnInput};
use solution_01::FirstSolution;
use std::{env::args, error::Error, time::Duration};
use types::Solution;

pub type Result<T, E = Box<dyn Error>> = std::result::Result<T, E>;

pub const TIME_LIMIT: Duration = Duration::from_millis(5800);

fn main() {
    if let Some(solution_id) = args().nth(1) {
        match &*solution_id {
            "1" => run::<FirstSolution>(),
            _ => panic!("unknown solution"),
        }
    } else {
        run::<FirstSolution>();
    }
}

fn run<S: Solution>() {
    let input = InitInput::read();
    let mut brain = S::init(input);
    // FIXME: remove this loop if the contest is not interactive.
    loop {
        let input = TurnInput::read();
        let plan = brain.think(input);
        println!("{}", plan);
    }
}
