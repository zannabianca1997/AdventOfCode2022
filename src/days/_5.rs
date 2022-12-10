use std::{error::Error, fmt::Display};

use super::PuzzleResult;

// use regex::Regex;

#[derive(Debug)]
enum InputError {
    NoBlankLine,
    NoDrawing,
    UnreadableMove(String),
    StackEmptied,
    StackEmptyAtTheEnd,
}

impl Error for InputError {}
impl Display for InputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Copy)]
struct Move {
    quantity: u8,
    from: u8,
    to: u8,
}

fn parse_drawing(input: &str) -> Result<Vec<Vec<char>>, InputError> {
    // splitting lines
    let (header, stacks_lines) = {
        let mut lines = input.lines().rev().filter(|l| l.trim() != "");
        let header = lines.next().ok_or(InputError::NoDrawing)?;
        (header, lines.collect::<Vec<_>>())
    };
    // counting the number and position of the rows
    let col_pos: Vec<_> = header
        .chars()
        .enumerate()
        .filter(|&(_, ch)| '1' <= ch && ch <= '9')
        .map(|(pos, ch)| (pos, ch.to_digit(10).unwrap() - 1))
        .collect();
    // making the stacks
    let mut stacks: Vec<Vec<char>> = (0..col_pos.len())
        .map(|_| Vec::with_capacity(stacks_lines.len()))
        .collect();
    // filling them
    for stack_line in stacks_lines {
        for (pos, col) in col_pos.iter() {
            if let Some(ch) = stack_line.chars().nth(*pos) {
                if ch != ' ' {
                    stacks[*col as usize].push(ch)
                }
            }
        }
    }

    Ok(stacks)
}
fn parse_moves(input: &str) -> Result<Vec<Move>, InputError> {
    /*
        // This is a cleaner REGEX solution. Sadly, also a lot slower

        lazy_static! {
            static ref RE: Regex = Regex::new(r"move (\d+) from (\d) to (\d)").unwrap();
        }
        Result::from_iter(input.trim().lines().map(|line| {
            RE.captures(line)
                .ok_or(InputError::UnreadableMove(line.to_string()))
                .map(|m| Move {
                    quantity: m.get(1).unwrap().as_str().parse().unwrap(),
                    from: m.get(2).unwrap().as_str().parse::<u8>().unwrap() - 1,
                    to: m.get(3).unwrap().as_str().parse::<u8>().unwrap() - 1,
                })
        }))
    */
    Result::from_iter(input.trim().lines().map(|line| {
        line.trim()
            // removing move
            .strip_prefix("move")
            // splitting parts
            .and_then(|line| line.split_once("from"))
            .and_then(|(qt, mov)| mov.split_once("to").map(|(from, to)| (qt, from, to)))
            // converting into ints
            .and_then(|(qt, from, to)| qt.trim().parse::<u8>().ok().map(|qt| (qt, from, to)))
            .and_then(|(qt, from, to)| {
                from.trim()
                    .parse::<u8>()
                    .ok()
                    .map(|from| (qt, from - 1, to))
            })
            .and_then(|(qt, from, to)| to.trim().parse::<u8>().ok().map(|to| (qt, from, to - 1)))
            // mapping to proprietary type
            .map(|(quantity, from, to)| Move { quantity, from, to })
            .ok_or(InputError::UnreadableMove(String::from(line)))
    }))
}

fn parse_input(input: &str) -> Result<(Vec<Vec<char>>, Vec<Move>), InputError> {
    // splitting the drawing from the move set
    let (drawing, moves) = input.split_once("\n\n").ok_or(InputError::NoBlankLine)?;
    let drawing = parse_drawing(drawing)?;
    let moves = parse_moves(moves)?;
    Ok((drawing, moves))
}

fn stack_tops(stacks: Vec<Vec<char>>) -> Result<String, InputError> {
    Result::from_iter(stacks.into_iter().map(|stack| {
        stack
            .last()
            .map(|ch| *ch)
            .ok_or(InputError::StackEmptyAtTheEnd)
    }))
}

pub fn part1(input: &str) -> Result<PuzzleResult, Box<dyn Error>> {
    let (mut stacks, moves) = parse_input(input)?;
    for mov in moves {
        for _ in 0..mov.quantity {
            let item = stacks[mov.from as usize]
                .pop()
                .ok_or(InputError::StackEmptied)?;
            stacks[mov.to as usize].push(item)
        }
    }
    stack_tops(stacks)
        .map(|s| PuzzleResult::Textual(s))
        .map_err(|e| e.into())
}

pub fn part2(input: &str) -> Result<PuzzleResult, Box<dyn Error>> {
    let (mut stacks, moves) = parse_input(input)?;
    for mov in moves {
        let mut items = vec![];
        for _ in 0..mov.quantity {
            items.push(
                stacks[mov.from as usize]
                    .pop()
                    .ok_or(InputError::StackEmptied)?,
            )
        }
        while let Some(item) = items.pop() {
            stacks[mov.to as usize].push(item)
        }
    }
    stack_tops(stacks)
        .map(|s| PuzzleResult::Textual(s))
        .map_err(|e| e.into())
}
