use itertools::{izip, Itertools};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "local", derive(serde::Serialize, serde::Deserialize))]
pub struct InitInput {
    // #[cfg_attr(feature = "local", serde(skip))]
    // _example: (),
}

impl InitInput {
    pub fn description_keys() -> Vec<&'static str> {
        vec![]
    }

    pub fn description_values(&self) -> Vec<String> {
        vec![]
    }

    pub fn describe(&self) -> String {
        izip!(Self::description_keys(), self.description_values(),)
            .map(|(key, value)| format!("{key} = {value}"))
            .join(", ")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TurnInput {}

impl TurnInput {}

#[derive(Debug, Clone)]
pub struct Output {
    pub operations: Vec<Operation>,
}

impl Output {}

#[derive(Debug, Clone)]
pub enum Operation {}

impl Operation {}

impl fmt::Display for Operation {
    fn fmt(&self, _b: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
