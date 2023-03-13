use std::cmp::Reverse;

use itertools::Itertools;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use crate::domain::entities::day::Day;

fn preprocessor(data: String) -> Vec<Vec<u32>> {
    data.split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|item| item.parse().expect("All item calories should be numbers"))
                .collect()
        })
        .collect()
}

fn part1(data: Vec<Vec<u32>>) -> String {
    data.par_iter()
        .map(|elf| elf.par_iter().sum::<u32>())
        .max()
        .expect("Max calories should exist in problem set")
        .to_string()
}

fn part2(data: Vec<Vec<u32>>) -> String {
    data
        .par_iter()
        .map(|elf| elf.par_iter().sum())
        .collect::<Vec<_>>()
        .iter()
        .sorted_by_key(|k| Reverse(*k))
        .take(3)
        .sum::<u32>()
        .to_string()
}

pub fn get_day() -> Day<Vec<Vec<u32>>> {
    Day::new(preprocessor, Some(part1), Some(part2))
}
