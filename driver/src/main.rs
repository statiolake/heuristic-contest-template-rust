use crate::brain::Brain;
use input::Input;
use std::{error::Error, time::Duration};

pub mod brain;
pub mod input;
pub mod simann;

pub type Result<T, E = Box<dyn Error>> = std::result::Result<T, E>;

pub const TIME_LIMIT: Duration = Duration::from_millis(5800);

fn main() {
    let input = Input::read();
    let mut brain = Brain::init(input);
    let plan = brain.think();
    println!("{}", plan);
}
