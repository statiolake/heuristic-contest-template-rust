use brain::Brain;
use io::{InitInput, TurnInput};
use std::{error::Error, time::Duration};

pub type Result<T, E = Box<dyn Error>> = std::result::Result<T, E>;

pub const TIME_LIMIT: Duration = Duration::from_millis(5800);

fn main() {
    let input = InitInput::read();
    let mut brain = Brain::init(input);
    let input = TurnInput::read();
    let plan = brain.think(input);
    println!("{}", plan);
}
