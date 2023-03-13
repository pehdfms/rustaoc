use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::{
    fs, io,
    path::{Path, PathBuf},
};
use thiserror::Error;

use reqwest::Url;

pub static CONFIG: Lazy<Config> =
    Lazy::new(|| load_config_file(Path::new("data/config.txt")).unwrap_or(Config::default()));

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    aoc_base_url: Url,
    session: String,
    storage_location: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            aoc_base_url: Url::parse("https://adventofcode.com").expect("Hardcoded"),
            session: "".into(),
            storage_location: Path::new("data").into(),
        }
    }
}

#[derive(Error, Debug)]
pub enum ConfigLoadError {
    #[error("Could not load file")]
    FileLoadError(#[from] io::Error),
    #[error("Could not deserialize config")]
    DeserializationError(#[from] bincode::Error),
}

pub fn load_config_file(path: &Path) -> Result<Config, ConfigLoadError> {
    let data = fs::read_to_string(path)?;
    Ok(bincode::deserialize_from(data.as_bytes())?)
}

impl<'a> Config {
    pub fn aoc_base_url(&self) -> &Url {
        &self.aoc_base_url
    }

    pub fn session(&'a self) -> &'a str {
        &self.session
    }

    pub fn cache_location(&self) -> PathBuf {
        self.storage_location.join("cache")
    }
}
