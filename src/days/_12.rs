use std::{
    collections::VecDeque,
    error::Error,
    ops::{Index, IndexMut},
};

use arrayvec::ArrayVec;

use super::PuzzleResult;

struct Grid<T> {
    height: usize,
    width: usize,
    memory: Box<[T]>,
}

impl<T> Grid<T> {
    fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x < self.width && y < self.height {
            Some(&self.memory[y * self.width + x])
        } else {
            None
        }
    }
    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        if x < self.width && y < self.height {
            Some(&mut self.memory[y * self.width + x])
        } else {
            None
        }
    }

    fn shape(&self) -> (usize, usize) {
        (self.width, self.height)
    }
}

impl<T> Grid<T>
where
    T: Clone,
{
    fn new(height: usize, width: usize, fill: T) -> Self {
        Self {
            height,
            width,
            memory: vec![fill; height * width].into_boxed_slice(),
        }
    }
    fn new_like<O>(other: &Grid<O>, fill: T) -> Self {
        Self::new(other.height, other.width, fill)
    }
}
impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        self.get(index.0, index.1)
            .unwrap_or_else(|| panic!("Index ({},{}) is out of range!", index.0, index.1))
    }
}
impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        self.get_mut(index.0, index.1)
            .unwrap_or_else(|| panic!("Index ({},{}) is out of range!", index.0, index.1))
    }
}

fn parse_input(input: &str) -> Result<(Grid<u8>, (usize, usize), (usize, usize)), Box<dyn Error>> {
    let lines: Vec<_> = input.trim().lines().collect();
    let height = lines.len();
    let width = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    let mut grid = Grid::new(height, width, 0);
    let mut start_pos = None;
    let mut end_pos = None;
    for (y, line) in lines.into_iter().rev().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            // check start and end pos
            let ch = match ch {
                'S' => {
                    start_pos = Some((x, y));
                    'a'
                }
                'E' => {
                    end_pos = Some((x, y));
                    'z'
                }
                c => c,
            };
            // map the rest
            grid[(x, y)] = ch as u8 - 'a' as u8
        }
    }
    if let (Some(start_pos), Some(end_pos)) = (start_pos, end_pos) {
        Ok((grid, start_pos, end_pos))
    } else {
        Err("Missing start or end position".into())
    }
}

fn neighbours(
    (x, y): (usize, usize),
    (max_x, max_y): (usize, usize),
) -> ArrayVec<(usize, usize), 4> {
    let mut neighbours: ArrayVec<_, 4> = ArrayVec::new();
    if x > 0 {
        neighbours.push((x - 1, y))
    }
    if y > 0 {
        neighbours.push((x, y - 1))
    }
    if x < max_x - 1 {
        neighbours.push((x + 1, y))
    }
    if y < max_y - 1 {
        neighbours.push((x, y + 1))
    }
    neighbours
}

pub fn part1(input: &str) -> Result<PuzzleResult, Box<dyn Error>> {
    let (heights, start, end) = parse_input(input)?;

    let mut visited = Grid::new_like(&heights, false);

    let mut to_check = VecDeque::new();
    to_check.push_back((start, 0)); // we can arrive at start with a lenght of 0
    visited[start] = true;

    while let Some((pos, len)) = to_check.pop_front() {
        for newpos in neighbours(pos, heights.shape()) {
            //check is reachable and unvisited
            if heights[newpos] > heights[pos] + 1 || visited[newpos] {
                // is unreachable or already visited
                continue;
            }
            // did we arrive?
            if newpos == end {
                return Ok(PuzzleResult::Numeric(len + 1));
            } else {
                // adding to the visited stuff, and marking for expanding
                to_check.push_back((newpos, len + 1));
                visited[newpos] = true;
            }
        }
    }
    Err("The end is not reachable".into())
}

pub fn part2(input: &str) -> Result<PuzzleResult, Box<dyn Error>> {
    let (heights, _, end) = parse_input(input)?;

    // Running the algorithm from the end position, searching for a square of height 'a'

    let mut visited = Grid::new_like(&heights, false);

    let mut to_check = VecDeque::new();
    to_check.push_back((end, 0)); // we can arrive at end with a lenght of 0
    visited[end] = true;

    while let Some((pos, len)) = to_check.pop_front() {
        for newpos in neighbours(pos, heights.shape()) {
            //check is reachable and unvisited
            if heights[pos] > heights[newpos] + 1 || visited[newpos] {
                // is unreachable or already visited
                continue;
            }
            // did we arrive?
            if heights[newpos] == 0 {
                return Ok(PuzzleResult::Numeric(len + 1));
            } else {
                // adding to the visited stuff, and marking for expanding
                to_check.push_back((newpos, len + 1));
                visited[newpos] = true;
            }
        }
    }
    Err("The 'a' level is not reachable".into())
}
