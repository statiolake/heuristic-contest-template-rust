use io::{InitInput, TurnInput};
use solution_01::Brain;
use std::{error::Error, time::Duration};

pub type Result<T, E = Box<dyn Error>> = std::result::Result<T, E>;

pub const TIME_LIMIT: Duration = Duration::from_millis(5800);

fn main() {
    let input = InitInput::read();
    let mut brain = Brain::init(input);
    // FIXME: remove this loop if the contest is not interactive.
    loop {
        let input = TurnInput::read();
        let plan = brain.think(input);
        println!("{}", plan);
    }
}
