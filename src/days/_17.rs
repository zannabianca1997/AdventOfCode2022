use std::error::Error;

use super::PuzzleResult;

const ROCKS: &[&[&[bool]]] = &[
    &[&[true, true, true, true]],
    &[
        &[false, true, false],
        &[true, true, true],
        &[false, true, false],
    ],
    &[
        &[true, true, true],
        &[false, false, true],
        &[false, false, true],
    ],
    &[&[true], &[true], &[true], &[true]],
    &[&[true, true], &[true, true]],
];

#[derive(Clone, Copy)]
enum PushDirection {
    LEFT,
    RIGHT,
}

fn parse_input(input: &str) -> Result<Box<[PushDirection]>, Box<dyn Error>> {
    use PushDirection::*;
    Result::<Vec<_>, _>::from_iter(input.trim().chars().map(|ch| match ch {
        '<' => Ok(LEFT),
        '>' => Ok(RIGHT),
        ch => Err(format!("Unrecognized char {ch}")),
    }))
    .map(|vec| vec.into_boxed_slice())
    .map_err(|err| err.into())
}

/// Check if the rock collided
fn collide(rock: &[&[bool]], rock_left: usize, rock_bottom: usize, lines: &[[bool; 7]]) -> bool {
    for (i, rock_line) in rock.iter().enumerate() {
        for (j, &v) in rock_line.iter().enumerate() {
            if v && lines[rock_bottom + i][rock_left + j] {
                return true;
            }
        }
    }
    false
}

pub fn part1(input: &str) -> Result<PuzzleResult, Box<dyn Error>> {
    let pushes = parse_input(input)?;
    let mut pushes_iter = pushes.iter().cycle();

    let mut lines: Vec<[bool; 7]> = vec![];
    let mut first_empty_line = 0;

    // dropping rocks
    for &rock in ROCKS.iter().cycle().take(2022) {
        let mut rock_bottom = first_empty_line + 3;
        let mut rock_left = 2;
        let rock_height = rock.len();
        let rock_width = rock[0].len();

        // print_state(rock, rock_left, rock_bottom, &lines);

        // adding additional lines as needed
        if lines.len() < rock_bottom + rock_height {
            lines.resize(rock_bottom + rock_height, [false; 7])
        }

        // drop the rock until it rests
        'drop: loop {
            // false push
            match *pushes_iter.next().unwrap() {
                PushDirection::LEFT => {
                    if rock_left != 0 {
                        let new_left = rock_left - 1;
                        if !collide(rock, new_left, rock_bottom, &lines) {
                            rock_left = new_left
                        }
                    }
                }
                PushDirection::RIGHT => {
                    if rock_left + rock_width != 7 {
                        let new_left = rock_left + 1;
                        if !collide(rock, new_left, rock_bottom, &lines) {
                            rock_left = new_left
                        }
                    }
                }
            }
            // Drop down
            if rock_bottom == 0 {
                break 'drop;
            }
            let new_bottom = rock_bottom - 1;
            if collide(rock, rock_left, new_bottom, &lines) {
                break 'drop;
            }
            rock_bottom = new_bottom;
        }

        // Rock has come to a rest. adding it to the lines...
        for (i, line) in rock.iter().enumerate() {
            for (j, &v) in line.iter().enumerate() {
                lines[rock_bottom + i][rock_left + j] |= v;
            }
        }
        // measuring heigth of the tower...
        first_empty_line = lines.len();
        while lines[first_empty_line - 1] == [false; 7] {
            first_empty_line -= 1;
        }
    }

    // measuring heigth of the tower...
    Ok(PuzzleResult::Numeric((first_empty_line) as _))
}

fn print_state(rock: &[&[bool]], rock_left: usize, rock_bottom: usize, lines: &[[bool; 7]]) {
    for h in (0..lines.len()).rev() {
        print!("|");
        for x in 0..7 {
            if rock_bottom <= h
                && h < rock_bottom + rock.len()
                && rock_left <= x
                && x < rock_left + rock[h - rock_bottom].len()
                && rock[h - rock_bottom][x - rock_left]
            {
                print!("@");
            }
            if lines[h][x] {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("|");
    }
    println!("+-------+");
    println!();
}

pub fn part2(input: &str) -> Result<PuzzleResult, Box<dyn Error>> {
    let pushes = parse_input(input)?;
    let mut pushes_iter = pushes.iter().enumerate().cycle().peekable();

    let mut lines: Vec<[bool; 7]> = vec![];
    let mut first_empty_line = 0;

    let mut sync_points = [((0, 0), 0, 0); 4];

    let mut jump_happened = false;
    let mut additional_height = 0;
    let mut additional_rocks = 0;

    // dropping rocks
    for (rock_total, (rock_num, &rock)) in ROCKS.iter().enumerate().cycle().enumerate() {
        if rock_total + additional_rocks == 1000000000000 {
            break;
        }
        let mut rock_bottom = first_empty_line + 3;
        let mut rock_left = 2;
        let rock_height = rock.len();
        let rock_width = rock[0].len();

        // adding additional lines as needed
        if lines.len() < rock_bottom + rock_height {
            lines.resize(rock_bottom + rock_height, [false; 7])
        }

        // drop the rock until it rests
        'drop: while let Some((push_num, &push)) = pushes_iter.next() {
            // lateral push
            match push {
                PushDirection::LEFT => {
                    if rock_left != 0 {
                        let new_left = rock_left - 1;
                        if !collide(rock, new_left, rock_bottom, &lines) {
                            rock_left = new_left
                        }
                    }
                }
                PushDirection::RIGHT => {
                    if rock_left + rock_width != 7 {
                        let new_left = rock_left + 1;
                        if !collide(rock, new_left, rock_bottom, &lines) {
                            rock_left = new_left
                        }
                    }
                }
            }
            // Drop down
            if rock_bottom == 0 {
                break 'drop;
            }
            let new_bottom = rock_bottom - 1;
            if collide(rock, rock_left, new_bottom, &lines) {
                break 'drop;
            }
            rock_bottom = new_bottom;
        }

        // Rock has come to a rest. adding it to the lines...
        for (i, line) in rock.iter().enumerate() {
            for (j, &v) in line.iter().enumerate() {
                lines[rock_bottom + i][rock_left + j] |= v;
            }
        }
        // measuring heigth of the tower...
        first_empty_line = lines.len();
        while lines[first_empty_line - 1] == [false; 7] {
            first_empty_line -= 1;
        }
    }

    // measuring heigth of the tower...
    Ok(PuzzleResult::Numeric(
        (first_empty_line + additional_height) as _,
    ))
}
