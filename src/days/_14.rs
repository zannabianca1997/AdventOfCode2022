use std::{
    error::Error,
    fmt::Display,
    num::ParseIntError,
    ops::{Index, IndexMut},
};

use super::PuzzleResult;
#[derive(Debug)]
enum ParseError {
    MissingComma,
    ParseIntError(ParseIntError),
}
impl From<ParseIntError> for ParseError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseIntError(value)
    }
}
impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::MissingComma => write!(f, "Missing comma"),
            ParseError::ParseIntError(err) => write!(f, "{err}"),
        }
    }
}
impl Error for ParseError {}

fn parse_input(input: &str) -> Result<Vec<Vec<(isize, isize)>>, ParseError> {
    Result::from_iter(input.trim().lines().map(|line| {
        Result::from_iter(line.split("->").map(|pos| {
            pos.split_once(',')
                .ok_or(ParseError::MissingComma)
                .and_then(|(p1, p2)| {
                    p1.trim()
                        .parse::<isize>()
                        .and_then(|p1| p2.trim().parse::<isize>().map(|p2| (p1, p2)))
                        .map_err(|err| err.into())
                })
        }))
    }))
}

struct Grid<T> {
    height: usize,
    width: usize,
    memory: Box<[T]>,
}

impl<T> Grid<T> {
    fn get(&self, x: isize, y: isize) -> Option<&T> {
        if x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height {
            Some(&self.memory[(y as usize) * self.width + x as usize])
        } else {
            None
        }
    }
    fn get_mut(&mut self, x: isize, y: isize) -> Option<&mut T> {
        if x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height {
            Some(&mut self.memory[(y as usize) * self.width + x as usize])
        } else {
            None
        }
    }
}

impl<T> Grid<T>
where
    T: Clone,
{
    fn new(width: usize, height: usize, fill: T) -> Self {
        Self {
            height,
            width,
            memory: vec![fill; height * width].into_boxed_slice(),
        }
    }
}
impl<T> Index<(isize, isize)> for Grid<T> {
    type Output = T;

    fn index(&self, index: (isize, isize)) -> &Self::Output {
        self.get(index.0, index.1)
            .unwrap_or_else(|| panic!("Index ({},{}) is out of range!", index.0, index.1))
    }
}
impl<T> IndexMut<(isize, isize)> for Grid<T> {
    fn index_mut(&mut self, index: (isize, isize)) -> &mut Self::Output {
        self.get_mut(index.0, index.1)
            .unwrap_or_else(|| panic!("Index ({},{}) is out of range!", index.0, index.1))
    }
}

fn make_field(
    input: Vec<Vec<(isize, isize)>>,
    drop_pos: (isize, isize),
    floor: bool,
) -> Result<(Grid<bool>, (isize, isize)), Box<dyn Error>> {
    let (min_x, max_x, min_y, max_y) = input.iter().flat_map(|line| line.iter()).fold(
        (drop_pos.0, drop_pos.0, drop_pos.1, drop_pos.1),
        |(min_x, max_x, min_y, max_y), (x, y)| {
            (min_x.min(*x), max_x.max(*x), min_y.min(*y), max_y.max(*y))
        },
    );
    let (min_x, max_x, min_y, max_y) = if floor {
        let floor_y = max_y + 2;
        let drop = floor_y - drop_pos.1;
        // open enough space to the sides for the mound
        let min_x = drop_pos.0 - drop;
        let max_x = drop_pos.0 + drop;
        // open space for the floor
        let max_y = max_y + 1;
        (min_x, max_x, min_y, max_y)
    } else {
        // open a little space to let it drop at the side
        let min_x = min_x - 1;
        let max_x = max_x + 1;
        (min_x, max_x, min_y, max_y)
    };
    // move drop pos to the relative place
    let drop_pos = (drop_pos.0 - min_x, drop_pos.1 - min_y);

    let mut field = Grid::new(
        (max_x + 1 - min_x) as usize,
        (max_y + 1 - min_y) as usize,
        false,
    );
    for line in input {
        for i in 0..line.len() - 1 {
            let (x1, y1) = line[i];
            let (x2, y2) = line[i + 1];
            if y1 == y2 {
                let y = y1 - min_y;
                for x in x1.min(x2)..=x1.max(x2) {
                    let x = x - min_x;
                    field[(x, y)] = true;
                }
            } else if x1 == x2 {
                let x = x1 - min_x;
                for y in y1.min(y2)..=y1.max(y2) {
                    let y = y - min_y;
                    field[(x, y)] = true;
                }
            } else {
                return Err("Line is not orthogonal".into());
            }
        }
    }
    Ok((field, drop_pos))
}

pub fn part1(input: &str) -> Result<PuzzleResult, Box<dyn Error>> {
    let input = parse_input(input)?;
    let drop_pos: (isize, isize) = (500, 0);
    let (mut field, drop_pos) = make_field(input, drop_pos, false)?;

    let mut deposited_grains = 0;
    'grains: loop {
        // drop a grain
        let mut pos = drop_pos;
        if field[pos] == true {
            return Err("Sand filled to the drop start".into());
        }
        'fall: loop {
            match (
                field.get(pos.0 - 1, pos.1 + 1),
                field.get(pos.0, pos.1 + 1),
                field.get(pos.0 + 1, pos.1 + 1),
            ) {
                (None, None, None) => break 'grains, // reached the bottom, fall infinitely

                (_, Some(false), _) => pos.1 += 1, // drop down
                (Some(false), Some(true), _) => {
                    pos.1 += 1;
                    pos.0 -= 1
                } // drop right
                (Some(true) | None, Some(true), Some(false)) => {
                    pos.1 += 1;
                    pos.0 += 1
                } // drop left

                (Some(true) | None, Some(true), Some(true) | None) => {
                    deposited_grains += 1;
                    field[pos] = true;
                    break 'fall;
                } // rest

                _ => unreachable!(),
            }
        }
    }

    Ok(PuzzleResult::Numeric(deposited_grains))
}

pub fn part2(input: &str) -> Result<PuzzleResult, Box<dyn Error>> {
    let input = parse_input(input)?;
    let drop_pos: (isize, isize) = (500, 0);
    let (mut field, drop_pos) = make_field(input, drop_pos, true)?;

    let mut deposited_grains = 0;
    'grains: loop {
        // drop a grain
        let mut pos = drop_pos;
        if field[pos] == true {
            break 'grains;
        }
        'fall: loop {
            match (
                field.get(pos.0 - 1, pos.1 + 1),
                field.get(pos.0, pos.1 + 1),
                field.get(pos.0 + 1, pos.1 + 1),
            ) {
                (_, Some(false), _) => pos.1 += 1, // drop down
                (Some(false), Some(true), _) => {
                    pos.1 += 1;
                    pos.0 -= 1
                } // drop right
                (Some(true) | None, Some(true), Some(false)) => {
                    pos.1 += 1;
                    pos.0 += 1
                } // drop left

                (Some(true) | None, Some(true) | None, Some(true) | None) => {
                    deposited_grains += 1;
                    field[pos] = true;
                    break 'fall;
                } // rest

                _ => unreachable!(),
            }
        }
    }

    Ok(PuzzleResult::Numeric(deposited_grains))
}
