use std::{
    error::Error,
    ops::{Index, IndexMut},
};

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

    fn iter(&self) -> impl Iterator<Item = &T> {
        self.memory.iter()
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

fn parse_input(input: &str) -> Result<Grid<u8>, Box<dyn Error>> {
    let lines: Vec<_> = input.trim().lines().collect();
    let height = lines.len();
    let width = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    let mut grid = Grid::new(height, width, 0);
    for (y, line) in lines.into_iter().rev().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            grid[(x, y)] = ch.to_digit(10).ok_or_else(|| -> Box<dyn Error> {
                format!("{ch} is not a valid height!").into()
            })? as u8;
        }
    }

    Ok(grid)
}

pub fn part1(input: &str) -> Result<PuzzleResult, Box<dyn Error>> {
    let heights = parse_input(input)?;
    let (max_height, max_width) = heights.shape();
    let mut visible = Grid::new_like(&heights, false);

    for x in 0..max_width {
        // top -> down
        let mut min_h = heights[(x, 0)];
        visible[(x, 0)] = true;
        for y in 1..max_height {
            if heights[(x, y)] > min_h {
                min_h = heights[(x, y)];
                visible[(x, y)] = true;
            }
        }
        // bottom -> up
        let mut min_h = heights[(x, max_height - 1)];
        visible[(x, max_height - 1)] = true;
        for y in (0..(max_height - 1)).rev() {
            if heights[(x, y)] > min_h {
                min_h = heights[(x, y)];
                visible[(x, y)] = true;
            }
        }
    }
    for y in 0..max_height {
        // left -> right
        let mut min_h = heights[(0, y)];
        visible[(0, y)] = true;
        for x in 1..max_width {
            if heights[(x, y)] > min_h {
                min_h = heights[(x, y)];
                visible[(x, y)] = true;
            }
        }
        // right -> left
        let mut min_h = heights[(max_width - 1, y)];
        visible[(max_width - 1, y)] = true;
        for x in (0..(max_width - 1)).rev() {
            if heights[(x, y)] > min_h {
                min_h = heights[(x, y)];
                visible[(x, y)] = true;
            }
        }
    }

    Ok(PuzzleResult::Numeric(
        visible.iter().filter(|v| **v).count() as i64,
    ))
}

pub fn part2(input: &str) -> Result<PuzzleResult, Box<dyn Error>> {
    let heights = parse_input(input)?;
    let (max_heigth, max_width) = heights.shape();

    let mut max_scenic_score = 0;
    for tx in 0..max_width {
        for ty in 0..max_heigth {
            let treehouse_height = heights[(tx, ty)];
            let scenic_score: usize =
                if tx == 0 || ty == 0 || tx == max_width - 1 || ty == max_heigth - 1 {
                    0
                } else {
                    (
                        // up
                        {
                        let mut ray_len = 0;
                        for y in (ty + 1)..max_heigth {
                            ray_len += 1;
                            if heights[(tx, y)] >= treehouse_height {
                                break;
                            }
                        }
                        ray_len
                    } *
                    // down
                    {
                        let mut ray_len = 0;
                        for y in (0..=(ty - 1)).rev() {
                            ray_len += 1;
                            if heights[(tx, y)] >= treehouse_height {
                                break;
                            }
                        }
                        ray_len
                    } *
                    // left
                    {
                        let mut ray_len = 0;
                        for x in (tx + 1)..max_width {
                            ray_len += 1;
                            if heights[(x, ty)] >= treehouse_height {
                                break;
                            }
                        }
                        ray_len
                    } *
                    // right
                    {
                        let mut ray_len = 0;
                        for x in (0..=(tx - 1)).rev() {
                            ray_len += 1;
                            if heights[(x, ty)] >= treehouse_height {
                                break;
                            }
                        }
                        ray_len
                    }
                    )
                };

            if scenic_score > max_scenic_score {
                max_scenic_score = scenic_score
            }
        }
    }
    Ok(PuzzleResult::Numeric(max_scenic_score as i64))
}
