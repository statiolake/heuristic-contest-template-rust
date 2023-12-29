extern crate io;
extern crate types;

macro_rules! define_solutions {
    ($($modname:ident::$solution:ident),*$(,)?) => {
        $(pub mod $modname;)*

        pub fn create_solution(name: &str, input: io::InitInput) -> Option<Box<dyn types::Solution>> {
            $(
                if <$modname::$solution as types::Solution>::name() == name {
                    return Some(Box::new(<$modname::$solution as types::Solution>::init(input)));
                }
            )*

            None
        }

        pub fn get_solution_names() -> Vec<&'static str> {
            vec![
                $(
                    <$modname::$solution as types::Solution>::name(),
                )*
            ]
        }
    };
}

define_solutions![naive::NaiveSolution];
