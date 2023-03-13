use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use regex::Regex;

use crate::domain::entities::day::Day;

struct Range {
    start: u32,
    end: u32,
}

impl Range {
    pub fn contains(&self, other: &Range) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    pub fn contains_or_is_contained_by(&self, other: &Range) -> bool {
        self.contains(other) || other.contains(self)
    }

    pub fn overlaps(&self, other: &Range) -> bool {
        !(self.start > other.end || self.end < other.start)
    }
}

impl From<(u32, u32)> for Range {
    fn from(value: (u32, u32)) -> Self {
        Range {
            start: value.0,
            end: value.1,
        }
    }
}

type AssignmentRange = (u32, u32);
type SectionAssignmentPair = (AssignmentRange, AssignmentRange);

fn preprocessor(data: String) -> Vec<SectionAssignmentPair> {
    let re = Regex::new(r#"(\d+)-(\d+),(\d+)-(\d+)"#).expect("Hardcoded Regex");
    re.captures_iter(&data)
        .map(|cap| {
            let first_range: AssignmentRange = (
                cap.get(1)
                    .expect("Section Assignments should contain 4 numbers")
                    .as_str()
                    .parse()
                    .expect("Captured elements should be numbers"),
                cap.get(2)
                    .expect("Section Assignments should contain 4 numbers")
                    .as_str()
                    .parse()
                    .expect("Captured elements should be numbers"),
            );

            let second_range: AssignmentRange = (
                cap.get(3)
                    .expect("Section Assignments should contain 4 numbers")
                    .as_str()
                    .parse()
                    .expect("Captured elements should be numbers"),
                cap.get(4)
                    .expect("Section Assignments should contain 4 numbers")
                    .as_str()
                    .parse()
                    .expect("Captured elements should be numbers"),
            );

            (first_range, second_range)
        })
        .collect()
}

fn part1(data: Vec<SectionAssignmentPair>) -> String {
    data.par_iter()
        .filter(|pair| {
            let first: Range = pair.0.into();
            let second: Range = pair.1.into();

            first.contains_or_is_contained_by(&second)
        })
        .count()
        .to_string()
}

fn part2(data: Vec<SectionAssignmentPair>) -> String {
    data.par_iter()
        .filter(|pair| {
            let first: Range = pair.0.into();
            let second: Range = pair.1.into();

            first.overlaps(&second)
        })
        .count()
        .to_string()
}

pub fn get_day() -> Day<Vec<SectionAssignmentPair>> {
    Day::new(preprocessor, Some(part1), Some(part2))
}
