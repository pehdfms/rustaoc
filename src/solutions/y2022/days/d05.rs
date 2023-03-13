use itertools::Itertools;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use regex::Regex;

use crate::domain::entities::day::Day;

type CrateStack = Vec<char>;
type Instruction = (u32, usize, usize);
type ProcessedInput = (Vec<CrateStack>, Vec<Instruction>);

fn parse_drawing(drawing: String) -> Vec<CrateStack> {
    // reverse lines in the drawing so we look at it bottom up
    let mut drawing = drawing.lines().rev();

    // figure out the amount of stacks from the first line by parsing the last digit
    let stack_numbers = drawing.next().expect("Drawing section should not be empty");
    let drawing = drawing;

    let stack_count_parser = Regex::new(r#"\d+$"#).expect("Hardcoded Regex");
    let stack_count = stack_count_parser
        .captures(stack_numbers)
        .expect("Drawing section should include stack numbers")
        .get(1)
        .expect("Should have stack number at the end of the line");

    // generate a regex from the amount of stacks we got before with optional capture groups for each stack
    // iterate through captures with their index to map the capture to their stack
    // if the capture is None, skip, otherwise push it to the stack

    todo!()
}

fn parse_instructions(instructions: String) -> Vec<Instruction> {
    let instruction_parser =
        Regex::new(r#"move (\d+) from (\d+) to (\d+)"#).expect("Hardcoded Regex");

    instruction_parser
        .captures_iter(&instructions)
        .map(|cap| {
            let times: u32 = cap
                .get(1)
                .expect("Instruction should be composed of three values")
                .as_str()
                .parse()
                .expect("These values should be numbers");

            let from: usize = cap
                .get(2)
                .expect("Instruction should be composed of three values")
                .as_str()
                .parse()
                .expect("These values should be numbers");

            let to: usize = cap
                .get(3)
                .expect("Instruction should be composed of three values")
                .as_str()
                .parse()
                .expect("These values should be numbers");

            (times, from, to)
        })
        .collect_vec()
}

fn preprocessor(data: String) -> ProcessedInput {
    let mut split_data = data.split("\n\n");

    let drawing = split_data
        .next()
        .expect("Input should have drawing and instructions separated by an empty line");

    let instructions = split_data
        .next()
        .expect("Input should have drawing and instructions separated by an empty line");

    (
        parse_drawing(drawing.to_string()),
        parse_instructions(instructions.to_string()),
    )
}

struct Supplies {
    stacks: Vec<CrateStack>,
}

impl Supplies {
    fn run_instruction(&mut self, instruction: MoveInstruction) {
        for movement in 0..instruction.times {
            let popped_crate = self.stacks[instruction.from - 1]
                .pop()
                .expect("No move instruction should take out more crates than exist");
            self.stacks[instruction.to - 1].push(popped_crate);
        }
    }

    fn peek(&self, stack: usize) -> char {
        **self.stacks[stack]
            .iter()
            .peekable()
            .peek()
            .expect("Peek instruction should always be in bounds")
    }

    pub fn top_crates(&self) -> String {
        let mut result: String = String::default();

        for idx in 0..self.stacks.len() {
            result.push(self.peek(idx));
        }

        result
    }
}

impl From<Vec<CrateStack>> for Supplies {
    fn from(value: Vec<CrateStack>) -> Self {
        Supplies { stacks: value }
    }
}

struct MoveInstruction {
    times: u32,
    from: usize,
    to: usize,
}

impl From<Instruction> for MoveInstruction {
    fn from(value: Instruction) -> Self {
        MoveInstruction {
            times: value.0,
            from: value.1,
            to: value.2,
        }
    }
}

fn part1(data: ProcessedInput) -> String {
    let mut supplies: Supplies = data.0.into();
    let instructions: Vec<MoveInstruction> = data
        .1
        .par_iter()
        .map(|instruction| MoveInstruction::from(*instruction))
        .collect::<Vec<MoveInstruction>>();

    for instruction in instructions {
        supplies.run_instruction(instruction);
    }

    supplies.top_crates()
}

fn part2(data: ProcessedInput) -> String {
    todo!()
}

pub fn get_day() -> Day<ProcessedInput> {
    Day::new(preprocessor, Some(part1), Some(part2))
}
