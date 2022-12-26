use std::error::Error;

use super::PuzzleResult;

fn parse_snafu(val: &str) -> Result<isize, Box<dyn Error>> {
    val.chars()
        .rev()
        .enumerate()
        .map(|(pow, ch)| {
            match ch {
                '2' => Ok(2),
                '1' => Ok(1),
                '0' => Ok(0),
                '-' => Ok(-1),
                '=' => Ok(-2),
                ch => Err(format!("{ch:?} is not a valid SNAFU digit")),
            }
            .and_then(|v| {
                5isize
                    .checked_pow(pow as u32)
                    .ok_or_else(|| "SNAFU number too big".to_owned())
                    .map(|exp| v * exp)
            })
        })
        .try_fold(0, |sum, x| x.map(|x| x + sum).map_err(|err| err.into()))
}
fn format_snafu(n: isize) -> String {
    let (last_digit, borrow) = match n.rem_euclid(5) {
        0 => ('0', 0),
        1 => ('1', 0),
        2 => ('2', 0),
        3 => ('=', 1),
        4 => ('-', 1),
        _ => unreachable!(),
    };
    let rest = n.div_euclid(5) + borrow;
    let mut pre = if rest > 0 {
        format_snafu(rest)
    } else {
        String::new()
    };
    pre.push(last_digit);
    pre
}

pub fn part1(input: &str) -> Result<PuzzleResult, Box<dyn Error>> {
    Ok(PuzzleResult::Textual(format_snafu(
        input
            .trim()
            .lines()
            .map(|line| parse_snafu(line.trim()))
            .try_fold(0, |sum, x| x.map(|x| x + sum))?,
    )))
}
