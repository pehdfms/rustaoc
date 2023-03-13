use std::collections::HashSet;

use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use crate::domain::entities::day::Day;

fn preprocessor(data: String) -> Vec<(String, String)> {
    data.lines()
        .map(|line| {
            let rucksacks = line.split_at(line.len() / 2);
            (rucksacks.0.to_string(), rucksacks.1.to_string())
        })
        .collect()
}

fn get_value(item: char) -> u32 {
    let value_table = (b'a'..=b'z').chain(b'A'..=b'Z');

    value_table
        .map(|c| c as char)
        .position(|character| character == item)
        .expect("All characters in input should be letters") as u32
        + 1
}

fn part1(data: Vec<(String, String)>) -> String {
    data.par_iter()
        .map(|rucksacks| {
            let first: HashSet<char> = rucksacks.0.chars().collect();
            let second: HashSet<char> = rucksacks.1.chars().collect();

            let common_items = first.intersection(&second);
            common_items.map(|item| get_value(*item)).sum::<u32>()
        })
        .sum::<u32>()
        .to_string()
}

fn part2(data: Vec<(String, String)>) -> String {
    data.array_chunks::<3>()
        .inspect(|chunk| {
            dbg!(chunk);
        })
        .map(|rucksack_group| {
            let intersections = rucksack_group.clone().map(|rucksacks| {
                (rucksacks.0 + &rucksacks.1)
                    .chars()
                    .collect::<HashSet<char>>()
            });

            intersections
                .iter()
                .fold(intersections[0].clone(), |acc, set| {
                    acc.intersection(set).cloned().collect()
                })
                .iter()
                .map(|c| get_value(*c))
                .sum::<u32>()
        })
        .sum::<u32>()
        .to_string()
}

pub fn get_day() -> Day<Vec<(String, String)>> {
    Day::new(preprocessor, Some(part1), Some(part2))
}
