use std::{collections::HashSet, error::Error, fmt::Display, num::ParseIntError};

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    const fn delta(self) -> (isize, isize) {
        match self {
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}
#[derive(Clone, Copy)]
struct Link {
    tail: (isize, isize),
}
impl Link {
    fn pull(&mut self, head: (isize, isize)) {
        // updating the tail
        let dx = head.0 - self.tail.0;
        let dy = head.1 - self.tail.1;
        let (tdx, tdy) = match (dx, dy) {
            (-1, -1)
            | (-1, 0)
            | (-1, 1)
            | (0, -1)
            | (0, 0)
            | (0, 1)
            | (1, -1)
            | (1, 0)
            | (1, 1) => (0, 0), // nothing to do, already touching
            // straight moves
            (2, 0) => (1, 0),
            (-2, 0) => (-1, 0),
            (0, 2) => (0, 1),
            (0, -2) => (0, -1),
            // diagonal moves
            (2, -2) | (2, -1) | (1, -2) => (1, -1),
            (-1, -2) | (-2, -2) | (-2, -1) => (-1, -1),
            (1, 2) | (2, 1) | (2, 2) => (1, 1),
            (-2, 1) | (-1, 2) | (-2, 2) => (-1, 1),
            // too far, impossible
            _ => unreachable!(),
        };
        self.tail.0 += tdx;
        self.tail.1 += tdy;
    }
}

struct Rope<const LEN: usize> {
    links: [Link; LEN],
}
impl<const LEN: usize> Rope<LEN> {
    fn new() -> Self {
        Self {
            links: [Link { tail: (0, 0) }; LEN],
        }
    }
    fn pull(&mut self, head: (isize, isize)) {
        self.links[0].pull(head);
        for i in 1..LEN {
            self.links[i].pull(self.links[i - 1].tail)
        }
    }
    fn tail(&self) -> (isize, isize) {
        self.links[LEN - 1].tail
    }
}

#[derive(Debug, Clone)]
enum ParseError {
    MissingSpace(String),
    UnknowDirection(String),
    ParseIntError(ParseIntError),
}
impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl Error for ParseError {}

fn parse_input(input: &str) -> Result<Vec<(Direction, usize)>, ParseError> {
    Result::from_iter(input.trim().lines().map(|line| {
        line.split_once(" ")
            .ok_or(ParseError::MissingSpace(line.to_owned()))
            .and_then(|(p1, p2)| {
                p2.trim()
                    .parse::<usize>()
                    .map(|p2| (p1, p2))
                    .map_err(|err| ParseError::ParseIntError(err))
            })
            .and_then(|(p1, p2)| {
                let dir = match p1 {
                    "U" => Direction::Up,
                    "D" => Direction::Down,
                    "L" => Direction::Left,
                    "R" => Direction::Right,
                    dir => return Err(ParseError::UnknowDirection(dir.to_owned())),
                };
                Ok((dir, p2))
            })
    }))
}

pub fn part1(input: &str) -> Result<String, Box<dyn Error>> {
    let directions = parse_input(input)?;
    let mut head = (0, 0);
    let mut chain = Link { tail: head };

    let mut tail_positions = HashSet::new();
    tail_positions.insert(chain.tail);

    for (direction, times) in directions {
        let direction = direction.delta();
        for _ in 0..times {
            head.0 += direction.0;
            head.1 += direction.1;
            chain.pull(head);
            tail_positions.insert(chain.tail);
        }
    }

    Ok(tail_positions.len().to_string())
}

pub fn part2(input: &str) -> Result<String, Box<dyn Error>> {
    let directions = parse_input(input)?;
    let mut head = (0, 0);
    let mut chain: Rope<9> = Rope::new();

    let mut tail_positions = HashSet::new();
    tail_positions.insert(chain.tail());

    for (direction, times) in directions {
        let direction = direction.delta();
        for _ in 0..times {
            head.0 += direction.0;
            head.1 += direction.1;
            chain.pull(head);
            tail_positions.insert(chain.tail());
        }
    }

    Ok(tail_positions.len().to_string())
}
