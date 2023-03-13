use crate::{
    domain::{
        entities::{
            day::DayResults,
            year::{AocYear, GetDayError},
        },
        services::input_fetcher::InputFetcher,
    },
    solutions::get_year,
};
use thiserror::Error;

pub struct SolutionController;

#[derive(Error, Debug)]
pub enum GetSolutionError {
    #[error("Could not get input from website")]
    FetchInputError(#[from] reqwest::Error),
    #[error("Could not find solution")]
    FetchSolutionError(#[from] GetDayError),
}

impl SolutionController {
    pub fn get(year: AocYear, day: usize) -> Result<DayResults, GetSolutionError> {
        let input = InputFetcher::get_input(year, day)?;
        Ok(get_year(year).get_day(day)?.run(input))
    }
}
