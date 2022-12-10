use std::{error::Error, fmt::Display};

use super::PuzzleResult;

#[derive(Debug)]
enum InputError {
    OddLenght(String),
    NonThriceable,
    StrangeChar(char),
    NoCommon,
}

impl Error for InputError {}
impl Display for InputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Container(u64);
impl Container {
    fn add(self, item: char) -> Option<Self> {
        let priority = if 'a' <= item && item <= 'z' {
            Some(item as u8 - 'a' as u8 + 1)
        } else if 'A' <= item && item <= 'Z' {
            Some(item as u8 - 'A' as u8 + 27)
        } else {
            None
        };
        priority.map(|p| Self(self.0 | 1 << p))
    }
    fn commons(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
    fn empty(self) -> bool {
        self.0 == 0
    }
    fn first_priority(self) -> u8 {
        let mut p = 0;
        let mut v = self.0;
        while v % 2 == 0 {
            v >>= 1;
            p += 1;
        }
        p
    }
}
impl TryFrom<&str> for Container {
    type Error = InputError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut res = Self(0);
        for ch in value.chars() {
            res = res.add(ch).ok_or(InputError::StrangeChar(ch))?
        }
        Ok(res)
    }
}

fn read_input_compartments(input: &str) -> Result<Vec<(Container, Container)>, InputError> {
    Result::from_iter(input.split("\n").filter_map(|line| {
        let line = line.trim();
        if line == "" {
            None
        } else if line.chars().count() % 2 != 0 {
            Some(Err(InputError::OddLenght(String::from(line))))
        } else {
            Some({
                let (s1, s2) = line.split_at(line.chars().count() / 2);
                Container::try_from(s1).and_then(|s1| Container::try_from(s2).map(|s2| (s1, s2)))
            })
        }
    }))
}
fn read_input_groups(input: &str) -> Result<Vec<(Container, Container, Container)>, InputError> {
    let lines: Vec<_> = Result::from_iter(input.split("\n").filter_map(|line| {
        let line = line.trim();
        if line == "" {
            None
        } else {
            Some(Container::try_from(line))
        }
    }))?;
    if lines.len() % 3 != 0 {
        return Err(InputError::NonThriceable);
    }
    let groups = lines.len() / 3;
    let mut res = Vec::with_capacity(groups);
    let mut lines = lines.into_iter();
    for _ in 0..groups {
        res.push((
            lines.next().unwrap(),
            lines.next().unwrap(),
            lines.next().unwrap(),
        ))
    }
    assert!(lines.next() == None);
    Ok(res)
}

pub fn part1(input: &str) -> Result<PuzzleResult, Box<dyn Error>> {
    let input = read_input_compartments(input)?;
    let mut total = 0;
    for (c1, c2) in input {
        let common = Container::commons(c1, c2);
        if !common.empty() {
            total += common.first_priority() as i64;
        } else {
            return Err(InputError::NoCommon.into());
        }
    }
    Ok(total).map(|v: i64| PuzzleResult::Numeric(v))
}

pub fn part2(input: &str) -> Result<PuzzleResult, Box<dyn Error>> {
    let input = read_input_groups(input)?;
    let mut total = 0;
    for (c1, c2, c3) in input {
        let common = Container::commons(c1, c2).commons(c3);
        if !common.empty() {
            total += common.first_priority() as i64;
        } else {
            return Err(InputError::NoCommon.into());
        }
    }
    Ok(total).map(|v: i64| PuzzleResult::Numeric(v))
}
