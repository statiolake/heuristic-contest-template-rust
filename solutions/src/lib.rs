extern crate io;
extern crate types;

use io::InitInput;
use types::Solution;

pub mod naive;

use naive::NaiveSolution;

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
