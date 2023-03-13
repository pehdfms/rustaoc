use std::fmt::Display;

use thiserror::Error;

use super::day::DaySolutions;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub enum AocYear {
    Year2019 = 2019,
    Year2022 = 2022,
}

impl Display for AocYear {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AocYear::Year2019 => write!(f, "2019"),
            AocYear::Year2022 => write!(f, "2022"),
        }
    }
}

pub struct Year {
    days: Vec<Box<dyn DaySolutions>>,
}

#[derive(Error, Debug)]
pub enum GetDayError {
    #[error("Out of range: {0} - Advent of Code only has 25 challenge days")]
    OutOfRange(usize),
    #[error("Day {0} is valid but has no current solution")]
    Unsolved(usize),
}

impl Year {
    pub const fn new(days: Vec<Box<dyn DaySolutions>>) -> Year {
        Year { days }
    }

    pub fn get_day(&self, idx: usize) -> Result<&dyn DaySolutions, GetDayError> {
        if idx > 25 {
            return Err(GetDayError::OutOfRange(idx));
        }

        self.days
            .get(idx - 1)
            .map(|boxed| &**boxed)
            .ok_or(GetDayError::Unsolved(idx))
    }
}
