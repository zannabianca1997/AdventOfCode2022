use std::error::Error;

use super::PuzzleResult;

pub fn part1(input: &str) -> Result<PuzzleResult, Box<dyn Error>> {
    let input: Vec<_> = input.chars().collect();
    const WINDOW: usize = 4;
    'outer: for (pos, window) in input.windows(WINDOW).enumerate() {
        for i in 0..window.len() {
            for j in 0..i {
                if window[i] == window[j] {
                    continue 'outer;
                }
            }
        }
        return Ok(PuzzleResult::Numeric((pos + WINDOW) as i64));
    }
    Err("Marker not found".into())
}

pub fn part2(input: &str) -> Result<PuzzleResult, Box<dyn Error>> {
    let input: Vec<_> = input.chars().collect();
    const WINDOW: usize = 14;
    'outer: for (pos, window) in input.windows(WINDOW).enumerate() {
        for i in 0..window.len() {
            for j in 0..i {
                if window[i] == window[j] {
                    continue 'outer;
                }
            }
        }
        return Ok(PuzzleResult::Numeric((pos + WINDOW) as i64));
    }
    Err("Marker not found".into())
}
