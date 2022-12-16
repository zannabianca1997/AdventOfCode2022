use std::{error::Error, path::Path};

use phf_macros::phf_map;

mod _01;
mod _02;
mod _03;
mod _04;
mod _05;
mod _06;
mod _07;
mod _08;
mod _09;
mod _10;
mod _11;
mod _12;
mod _13;
mod _14;
mod _15;

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
pub type ReprFn = fn(&str, Path) -> Result<(), Box<dyn Error>>;

pub const DAYS: [(
    Option<SolveFn>,
    Option<SolveFn>,
    phf::Map<&'static str, ReprFn>,
); 25] = [
    (Some(_01::part1), Some(_01::part2), phf_map! {}),
    (Some(_02::part1), Some(_02::part2), phf_map! {}),
    (Some(_03::part1), Some(_03::part2), phf_map! {}),
    (Some(_04::part1), Some(_04::part2), phf_map! {}),
    (Some(_05::part1), Some(_05::part2), phf_map! {}),
    (Some(_06::part1), Some(_06::part2), phf_map! {}),
    (Some(_07::part1), Some(_07::part2), phf_map! {}),
    (Some(_08::part1), Some(_08::part2), phf_map! {}),
    (Some(_09::part1), Some(_09::part2), phf_map! {}),
    (Some(_10::part1), Some(_10::part2), phf_map! {}),
    (Some(_11::part1), Some(_11::part2), phf_map! {}),
    (Some(_12::part1), Some(_12::part2), phf_map! {}),
    (Some(_13::part1), Some(_13::part2), phf_map! {}),
    (Some(_14::part1), Some(_14::part2), phf_map! {}),
    (Some(_15::part1), Some(_15::part2), phf_map! {}),
    (None, None, phf_map! {}),
    (None, None, phf_map! {}),
    (None, None, phf_map! {}),
    (None, None, phf_map! {}),
    (None, None, phf_map! {}),
    (None, None, phf_map! {}),
    (None, None, phf_map! {}),
    (None, None, phf_map! {}),
    (None, None, phf_map! {}),
    (None, None, phf_map! {}),
];
