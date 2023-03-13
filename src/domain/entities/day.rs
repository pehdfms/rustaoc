use crate::domain::common::timed;

use super::{PreProcessor, Solution};

#[derive(Debug)]
pub enum Progress {
    FullySolved,
    PartlySolved,
    Unsolved,
}

pub struct Day<T: Clone> {
    preprocessor: PreProcessor<T>,
    part1: Option<Solution<T>>,
    part2: Option<Solution<T>>,
}

#[derive(Debug)]
pub struct PartResult {
    pub time_taken: u128,
    pub result: String,
}

#[derive(Debug)]
pub struct DayResults {
    pub processing_time: u128,
    pub cloning_time: u128,
    pub part1: Option<PartResult>,
    pub part2: Option<PartResult>,
    pub progress: Progress,
}

impl<T: Clone> Day<T> {
    pub fn new(
        preprocessor: PreProcessor<T>,
        part1: Option<Solution<T>>,
        part2: Option<Solution<T>>,
    ) -> Self {
        Self {
            preprocessor,
            part1,
            part2,
        }
    }
}

impl<T: Clone> DaySolutions for Day<T> {
    fn run(&self, data: String) -> DayResults {
        let (processing_time, processed_data) = timed(|| (self.preprocessor)(data));
        let (cloning_time, cloned_data) = timed(|| processed_data.clone());

        let part1_result = self.part1.map(|solve| timed(|| solve(processed_data)));
        let part2_result = self.part2.map(|solve| timed(|| solve(cloned_data)));

        DayResults {
            processing_time,
            cloning_time,
            part1: part1_result.map(|result| PartResult {
                time_taken: result.0,
                result: result.1,
            }),
            part2: part2_result.map(|result| PartResult {
                time_taken: result.0,
                result: result.1,
            }),
            progress: self.progress(),
        }
    }

    fn progress(&self) -> Progress {
        match (self.part1, self.part2) {
            (Some(_), Some(_)) => Progress::FullySolved,
            (Some(_), None) | (None, Some(_)) => Progress::PartlySolved,
            (None, None) => Progress::Unsolved,
        }
    }
}

pub trait DaySolutions {
    fn run(&self, data: String) -> DayResults;
    fn progress(&self) -> Progress;
}

impl Day<String> {
    #[must_use]
    pub fn without_preprocessor(
        part1: Option<Solution<String>>,
        part2: Option<Solution<String>>,
    ) -> Self {
        Self::new(|data| data, part1, part2)
    }
}
