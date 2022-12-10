use std::{error::Error, fmt::Display, num::ParseIntError};

use super::PuzzleResult;

#[derive(Debug)]
enum InputError {
    NoComma,
    NoLine,
    ParseIntError(ParseIntError),
}

impl Error for InputError {}
impl Display for InputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl From<ParseIntError> for InputError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseIntError(value)
    }
}

#[derive(Debug, Clone, Copy)]
struct SectionRange(i64, i64);
impl SectionRange {
    fn is_inside(self, other: SectionRange) -> bool {
        other.0 <= self.0 && self.1 <= other.1
    }
    fn overlap(self, other: SectionRange) -> bool {
        (other.0 <= self.0 && self.0 <= other.1)
            || (other.0 <= self.1 && self.1 <= other.1)
            || other.is_inside(self)
    }
}
impl TryFrom<&str> for SectionRange {
    type Error = InputError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (s1, s2) = value.split_once("-").ok_or(InputError::NoLine)?;
        let begin = s1.trim().parse()?;
        let end = s2.trim().parse()?;
        Ok(Self(begin, end))
    }
}

fn read_input(input: &str) -> Result<Vec<(SectionRange, SectionRange)>, InputError> {
    Result::from_iter(input.trim().lines().map(|line| {
        line.split_once(",")
            .ok_or(InputError::NoComma)
            .and_then(|(s1, s2)| {
                SectionRange::try_from(s1)
                    .and_then(|s1| SectionRange::try_from(s2).map(|s2| (s1, s2)))
            })
    }))
}

pub fn part1(input: &str) -> Result<PuzzleResult, Box<dyn Error>> {
    Ok(read_input(input)?
        .into_iter()
        .filter(|(r1, r2)| r1.is_inside(*r2) || r2.is_inside(*r1))
        .count() as i64)
    .map(|v: i64| PuzzleResult::Numeric(v))
}

pub fn part2(input: &str) -> Result<PuzzleResult, Box<dyn Error>> {
    Ok(read_input(input)?
        .into_iter()
        .filter(|(r1, r2)| r1.overlap(*r2))
        .count() as i64)
    .map(|v: i64| PuzzleResult::Numeric(v))
}
