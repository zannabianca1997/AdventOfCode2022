use std::error::Error;

mod _1;
mod _2;
mod _3;
mod _4;
mod _5;
mod _6;
mod _7;

pub type SolveFn = fn(&str) -> Result<String, Box<dyn Error>>;

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
    Unsolved,
    Unsolved,
    Unsolved,
    Unsolved,
    Unsolved,
    Unsolved,
    Unsolved,
];
