use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use regex::Regex;

use crate::domain::entities::day::Day;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum RPSVariant {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum RPSOutcome {
    Win,
    Draw,
    Loss,
}

impl RPSOutcome {
    pub fn derive_move(&self, opponent: RPSVariant) -> RPSVariant {
        match self {
            Self::Win => opponent.loses_against(),
            Self::Draw => opponent.draws_against(),
            Self::Loss => opponent.wins_against(),
        }
    }

    pub fn get_score(&self) -> u32 {
        match self {
            RPSOutcome::Win => 6,
            RPSOutcome::Draw => 3,
            RPSOutcome::Loss => 0,
        }
    }
}

impl From<char> for RPSOutcome {
    fn from(value: char) -> Self {
        match value {
            'X' => Self::Loss,
            'Y' => Self::Draw,
            'Z' => Self::Win,
            _ => unreachable!("There should only ever be three outcomes in the input data"),
        }
    }
}

impl RPSVariant {
    pub fn compare(&self, opponent: &RPSVariant) -> RPSOutcome {
        if self == opponent {
            RPSOutcome::Draw
        } else if self.can_beat(opponent) {
            RPSOutcome::Win
        } else {
            RPSOutcome::Loss
        }
    }

    pub fn can_beat(&self, opponent: &RPSVariant) -> bool {
        &self.wins_against() == opponent
    }

    pub fn loses_against(&self) -> RPSVariant {
        match self {
            RPSVariant::Rock => RPSVariant::Paper,
            RPSVariant::Paper => RPSVariant::Scissors,
            RPSVariant::Scissors => RPSVariant::Rock,
        }
    }

    pub fn wins_against(&self) -> RPSVariant {
        match self {
            RPSVariant::Rock => RPSVariant::Scissors,
            RPSVariant::Paper => RPSVariant::Rock,
            RPSVariant::Scissors => RPSVariant::Paper,
        }
    }

    pub fn draws_against(&self) -> RPSVariant {
        *self
    }

    pub fn get_score(&self) -> u32 {
        match self {
            RPSVariant::Rock => 1,
            RPSVariant::Paper => 2,
            RPSVariant::Scissors => 3,
        }
    }
}

impl From<char> for RPSVariant {
    fn from(value: char) -> Self {
        match value {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissors,
            _ => unreachable!("There should only ever be three variants in the input data"),
        }
    }
}

#[derive(Clone, Debug)]
struct Round {
    pub opponent: RPSVariant,
    pub you: RPSVariant,
}

impl Round {
    pub fn get_score(&self) -> u32 {
        let match_score = self.you.compare(&self.opponent).get_score();
        let your_score = self.you.get_score();
        your_score + match_score
    }
}

fn preprocessor(data: String) -> Vec<(char, char)> {
    let re = Regex::new(r#"([ABC]) ([XYZ])"#).expect("Hardcoded Regex");

    re.captures_iter(&data)
        .map(|cap| {
            let opponent = cap
                .get(1)
                .expect("Every round should have two moves")
                .as_str()
                .chars()
                .next()
                .expect("Every move should be only one character");

            let you = cap
                .get(2)
                .expect("Every round should have two moves")
                .as_str()
                .chars()
                .next()
                .expect("Every move should be only one character");

            (opponent, you)
        })
        .collect()
}

fn part1(data: Vec<(char, char)>) -> String {
    data.par_iter()
        .map(|round| {
            Round {
                opponent: RPSVariant::from(round.0),
                you: RPSVariant::from(round.1),
            }
            .get_score()
        })
        .sum::<u32>()
        .to_string()
}

fn part2(data: Vec<(char, char)>) -> String {
    data.par_iter()
        .map(|round| {
            let opponent = RPSVariant::from(round.0);

            Round {
                opponent,
                you: RPSOutcome::from(round.1).derive_move(opponent),
            }
            .get_score()
        })
        .sum::<u32>()
        .to_string()
}

pub fn get_day() -> Day<Vec<(char, char)>> {
    Day::new(preprocessor, Some(part1), Some(part2))
}
