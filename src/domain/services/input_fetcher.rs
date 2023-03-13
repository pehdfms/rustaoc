use std::{fs, path::PathBuf};

use reqwest::{blocking::Client, header::COOKIE};

use crate::{domain::entities::year::AocYear, Ignorable};

use super::config::CONFIG;

pub struct InputFetcher;

impl InputFetcher {
    pub fn get_input(year: AocYear, day: usize) -> Result<String, reqwest::Error> {
        Self::read_cache(year, day).map(Ok).unwrap_or_else(|_| {
            Self::fetch(year, day).inspect(|input| {
                Self::write_cache(year, day, input.clone()).ignore();
            })
        })
    }

    fn cache_file_path(year: AocYear, day: usize) -> PathBuf {
        CONFIG.cache_location().join(format!("{year}/day{day}.txt"))
    }

    fn write_cache(year: AocYear, day: usize, input: String) -> Result<(), std::io::Error> {
        fs::create_dir_all(CONFIG.cache_location().join(year.to_string()))?;
        fs::write(Self::cache_file_path(year, day), input)
    }

    fn read_cache(year: AocYear, day: usize) -> Result<String, std::io::Error> {
        fs::read_to_string(Self::cache_file_path(year, day))
    }

    fn fetch(year: AocYear, day: usize) -> Result<String, reqwest::Error> {
        Client::builder()
            .cookie_store(true)
            .build()?
            .get(
                CONFIG.aoc_base_url()
                    .join(format!("/{year}/day/{day}/input").as_str())
                    .expect("URL concatenation shouldn't be a problem here"),
            )
            .header(COOKIE, format!("session={}", CONFIG.session()))
            .send()?
            .text()
            .and_then(|text| {
                if text.contains("Puzzle inputs differ by user.") {
                    panic!("Tried to fetch input data from the advent of code website, but could not authenticate. Did you set up your session key correctly?")
                } else {
                    Ok(text)
                }
            })
    }
}
