use std::error::Error;

mod _1;
mod _10;
mod _11;
mod _12;
mod _13;
mod _14;
mod _2;
mod _3;
mod _4;
mod _5;
mod _6;
mod _7;
mod _8;
mod _9;

pub enum ResultRepr {
    Short(String),
    Multiline(String),
}

#[derive(Debug, Clone)]
pub enum PuzzleResult {
    Numeric(i64),
    Textual(String),
    AsciiArt(String),
}
impl PuzzleResult {
    pub fn repr(self) -> ResultRepr {
        use ResultRepr::*;
        match self {
            PuzzleResult::Numeric(v) => Short(v.to_string()),
            PuzzleResult::Textual(s) => Short(s),
            PuzzleResult::AsciiArt(s) => Multiline(s),
        }
    }
}

pub type SolveFn = fn(&str) -> Result<PuzzleResult, Box<dyn Error>>;

#[derive(Clone, Copy)]
#[allow(dead_code)]
pub enum SolveState {
    Unsolved,
    P1Done(SolveFn),
    Done(SolveFn, SolveFn),
}

use SolveState::*;

pub const DAYS: [SolveState; 25] = [
    Done(_1::part1, _1::part2),
    Done(_2::part1, _2::part2),
    Done(_3::part1, _3::part2),
    Done(_4::part1, _4::part2),
    Done(_5::part1, _5::part2),
    Done(_6::part1, _6::part2),
    Done(_7::part1, _7::part2),
    Done(_8::part1, _8::part2),
    Done(_9::part1, _9::part2),
    Done(_10::part1, _10::part2),
    Done(_11::part1, _11::part2),
    Done(_12::part1, _12::part2),
    Done(_13::part1, _13::part2),
    Done(_14::part1, _14::part2),
    Unsolved,
    Unsolved,
    Unsolved,
    Unsolved,
    Unsolved,
    Unsolved,
    Unsolved,
    Unsolved,
    Unsolved,
    Unsolved,
    Unsolved,
];
