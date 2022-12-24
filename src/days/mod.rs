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
mod _16;
mod _17;
mod _18;
mod _19;
mod _20;
mod _21;
mod _22;

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
    pub fn repr(&self) -> ResultRepr {
        use ResultRepr::*;
        match self {
            PuzzleResult::Numeric(v) => Short(v.to_string()),
            PuzzleResult::Textual(s) => Short(s.clone()),
            PuzzleResult::AsciiArt(s) => Multiline(s.clone()),
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
    (Some(_16::part1), Some(_16::part2), phf_map! {}),
    (Some(_17::part1), Some(_17::part2), phf_map! {}),
    (Some(_18::part1), Some(_18::part2), phf_map! {}),
    (Some(_19::part1), Some(_19::part2), phf_map! {}),
    (Some(_20::part1), Some(_20::part2), phf_map! {}),
    (Some(_21::part1), Some(_21::part2), phf_map! {}),
    (Some(_22::part1), Some(_22::part2), phf_map! {}),
    (None, None, phf_map! {}),
    (None, None, phf_map! {}),
    (None, None, phf_map! {}),
];
