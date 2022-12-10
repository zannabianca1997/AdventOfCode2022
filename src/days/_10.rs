use std::{error::Error, fmt::Display, num::ParseIntError};

use super::PuzzleResult;

#[derive(Clone, Copy)]
enum Instruction {
    Noop,
    AddX(isize),
}
impl Instruction {
    fn time_to_execute(&self) -> usize {
        match self {
            Instruction::Noop => 1,
            Instruction::AddX(_) => 2,
        }
    }
}

#[derive(Debug, Clone)]
enum ParseError {
    UnknowInstruction(String),
    UnexpectedArgument(String),
    ParseIntError(ParseIntError),
    MissingArgument,
}
impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl Error for ParseError {}

fn parse_input(input: &str) -> Result<Vec<Instruction>, ParseError> {
    Result::from_iter(input.trim().lines().map(|line| {
        let (instr, arg) = line
            .split_once(" ")
            .map(|(p1, p2)| (p1.trim(), Some(p2.trim())))
            .unwrap_or((line, None));
        match (instr, arg) {
            ("noop", None) => Ok(Instruction::Noop),
            ("noop", Some(a)) => Err(ParseError::UnexpectedArgument(a.to_owned())),
            ("addx", Some(a)) => a
                .parse()
                .map(|v| Instruction::AddX(v))
                .map_err(|err| ParseError::ParseIntError(err)),
            ("addx", None) => Err(ParseError::MissingArgument),
            (instr, _) => Err(ParseError::UnknowInstruction(instr.to_owned())),
        }
    }))
}

struct CRT {
    clock: usize,
    x: isize,
    running_instruction: Option<Instruction>,
    time_to_completion: usize,
    instruction_stack: Vec<Instruction>,
}
impl CRT {
    fn new(mut program: Vec<Instruction>) -> Self {
        program.reverse();
        let running_instruction = program.pop();
        let time_to_completion = running_instruction
            .map(|i| i.time_to_execute())
            .unwrap_or(0);
        Self {
            clock: 0,
            x: 1,
            running_instruction,
            time_to_completion,
            instruction_stack: program,
        }
    }
    fn step(&mut self) -> Option<isize> {
        if let Some(instr) = self.running_instruction {
            let x_during = self.x;
            self.clock += 1;
            self.time_to_completion -= 1;
            if self.time_to_completion == 0 {
                match instr {
                    Instruction::Noop => (),
                    Instruction::AddX(v) => self.x += v,
                }
                self.running_instruction = self.instruction_stack.pop();
                self.time_to_completion = self
                    .running_instruction
                    .map(|i| i.time_to_execute())
                    .unwrap_or(0);
            }
            Some(x_during)
        } else {
            None
        }
    }
}

pub fn part1(input: &str) -> Result<PuzzleResult, Box<dyn Error>> {
    let input = parse_input(input)?;
    let mut crt = CRT::new(input);
    let mut total = 0;
    while let Some(x) = crt.step() {
        let stregth = (crt.clock as isize) * x;
        if (crt.clock + 20) % 40 == 0 && crt.clock <= 220 {
            /*
            dbg!(crt.clock);
            dbg!(stregth);
            */
            total += stregth;
        }
    }
    Ok(PuzzleResult::Numeric(total as i64))
}

pub fn part2(input: &str) -> Result<PuzzleResult, Box<dyn Error>> {
    let input = parse_input(input)?;
    let mut crt = CRT::new(input);
    let mut screen = String::with_capacity(40 * 6);
    while let Some(sprite_pos) = crt.step() {
        let cycle = crt.clock;
        if ((cycle % 40) as isize - sprite_pos - 1).abs() <= 1 {
            // the pixel is inside the sprite
            screen.push('#')
        } else {
            // no pixel draw
            screen.push(' ')
        }
        if cycle % 40 == 0 {
            screen.push('\n')
        }
        if cycle == 40 * 6 {
            break;
        }
    }
    Ok(PuzzleResult::AsciiArt(screen))
}
