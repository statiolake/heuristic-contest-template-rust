extern crate io;
extern crate types;

use std::ops::ControlFlow;

use io::{InitInput, Output, TurnInput};

pub mod naive;

use naive::NaiveSolution;

pub trait Solution {
    fn name() -> &'static str
    where
        Self: Sized;

    fn init(input: InitInput) -> Self
    where
        Self: Sized;

    fn think(&mut self, turn: TurnInput) -> ControlFlow<Output, Output>;
}

macro_rules! define_solutions {
    ($($solution:ident),*$(,)?) => {
        pub fn create_solution(name: &str, input: InitInput) -> Option<Box<dyn Solution>> {
            $(
                if <$solution as Solution>::name() == name {
                    return Some(Box::new(<$solution as Solution>::init(input)));
                }
            )*

            None
        }

        pub fn get_solution_names() -> Vec<&'static str> {
            vec![
                $(
                    <$solution as Solution>::name(),
                )*
            ]
        }
    };
}

define_solutions![NaiveSolution];
