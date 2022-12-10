use std::{error::Error, fmt::Display, num::ParseIntError};

use super::PuzzleResult;

fn elves_backpacks(input: &str) -> Result<Vec<Vec<i64>>, ParseIntError> {
    Result::from_iter(input.split("\n\n").map(|pack| {
        Result::from_iter(
            pack.split("\n")
                .filter(|line| line.trim() != "")
                .map(|line| line.parse()),
        )
    }))
}

#[derive(Debug)]
enum InputError {
    Empty,
    LessThanThree,
}

impl Error for InputError {}
impl Display for InputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                InputError::Empty => "Empty input",
                InputError::LessThanThree => "Less than three backpacks",
            }
        )
    }
}

pub fn part1(input: &str) -> Result<PuzzleResult, Box<dyn Error>> {
    elves_backpacks(input)
        .map_err(|err| err.into())
        .and_then(|backpacks| {
            backpacks
                .into_iter()
                .map(|pack| pack.into_iter().sum())
                .max()
                .ok_or(InputError::Empty.into())
        })
        .map(|v: i64| PuzzleResult::Numeric(v))
}

pub fn part2(input: &str) -> Result<PuzzleResult, Box<dyn Error>> {
    elves_backpacks(input)
        .map_err(|err| err.into())
        .and_then(|backpacks| {
            let mut packs = backpacks
                .into_iter()
                .map(|pack| pack.into_iter().sum::<i64>());
            let mut largest_three = [
                packs.next().ok_or(InputError::LessThanThree)?,
                packs.next().ok_or(InputError::LessThanThree)?,
                packs.next().ok_or(InputError::LessThanThree)?,
            ];
            largest_three.sort();
            for pack in packs {
                if pack > largest_three[0] {
                    largest_three[0] = pack;
                    largest_three.sort();
                }
            }
            Ok(largest_three.into_iter().sum())
        })
        .map(|v: i64| PuzzleResult::Numeric(v))
}
