extern crate io;
extern crate referee;
extern crate solutions;

use io::traits::{ReadInput, WriteOutput};
use referee::{InitInput, TurnInput};
use solutions::create_solution;
use std::{env::args, error::Error, ops::ControlFlow, time::Duration};

pub type Result<T, E = Box<dyn Error>> = std::result::Result<T, E>;

pub const TIME_LIMIT: Duration = Duration::from_millis(5800);

fn main() {
    let name = if let Some(solution_name) = args().nth(1) {
        solution_name
    } else {
        solutions::get_solution_names()
            .last()
            .unwrap_or(&"naive")
            .to_string()
    };

    run(&name);
}

fn run(name: &str) {
    let input = InitInput::read();
    let mut brain =
        create_solution(name, input).unwrap_or_else(|| panic!("unknown solution: {}", name));

    loop {
        let input = TurnInput::read();
        match brain.think(input) {
            ControlFlow::Continue(output) => output.write(),
            ControlFlow::Break(output) => {
                output.write();
                break;
            }
        }
    }
}
