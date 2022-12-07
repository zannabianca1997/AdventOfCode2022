use std::{error::Error, fmt::Display};

#[derive(Debug, Clone, Copy)]
enum RPSMove {
    Rock,
    Paper,
    Scissor,
}
#[derive(Debug, Clone, Copy)]
enum Column2 {
    X,
    Y,
    Z,
}
#[derive(Debug, Clone, Copy)]
enum GameResult {
    Win,
    Lose,
    Draw,
}

impl RPSMove {
    fn against(self, p2move: Self) -> GameResult {
        use GameResult::*;
        use RPSMove::*;
        match (self, p2move) {
            (Rock, Rock) => Draw,
            (Rock, Paper) => Lose,
            (Rock, Scissor) => Win,
            (Paper, Rock) => Win,
            (Paper, Paper) => Draw,
            (Paper, Scissor) => Lose,
            (Scissor, Rock) => Lose,
            (Scissor, Paper) => Win,
            (Scissor, Scissor) => Draw,
        }
    }

    fn round_score(self, p2move: Self) -> i64 {
        let shape_score = match self {
            RPSMove::Rock => 1,
            RPSMove::Paper => 2,
            RPSMove::Scissor => 3,
        };
        let result_score = match self.against(p2move) {
            GameResult::Lose => 0,
            GameResult::Draw => 3,
            GameResult::Win => 6,
        };
        shape_score + result_score
    }
}

#[derive(Debug)]
enum InputError {
    NoSpace(String),
    WrongMove(String),
}

impl Error for InputError {}
impl Display for InputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn read_input(input: &str) -> Result<Vec<(RPSMove, Column2)>, InputError> {
    Result::from_iter(input.split("\n").filter_map(|line| {
        let line = line.trim();
        if let Some((p1, p2)) = line.split_once(" ") {
            use Column2::*;
            use RPSMove::*;

            let p1 = match p1 {
                "A" => Rock,
                "B" => Paper,
                "C" => Scissor,
                _ => return Some(Err(InputError::WrongMove(String::from(p1)))),
            };
            let p2 = match p2 {
                "X" => X,
                "Y" => Y,
                "Z" => Z,
                _ => return Some(Err(InputError::WrongMove(String::from(p2)))),
            };
            Some(Ok((p1, p2)))
        } else if line.trim() == "" {
            None // skip empty lines
        } else {
            Some(Err(InputError::NoSpace(String::from(line))))
        }
    }))
}

pub fn part1(input: &str) -> Result<String, Box<dyn Error>> {
    Ok(read_input(input)?
        .into_iter()
        .map(|(p2, p1)| {
            let p1 = match p1 {
                Column2::X => RPSMove::Rock,
                Column2::Y => RPSMove::Paper,
                Column2::Z => RPSMove::Scissor,
            };
            p1.round_score(p2)
        })
        .sum())
    .map(|v: i64| v.to_string())
}

pub fn part2(input: &str) -> Result<String, Box<dyn Error>> {
    Ok(read_input(input)?
        .into_iter()
        .map(|(p2, p1)| {
            use GameResult::*;
            use RPSMove::*;
            // converting into the needed result
            let p1 = match p1 {
                Column2::X => Lose,
                Column2::Y => Draw,
                Column2::Z => Win,
            };
            // finding the right move
            let p1 = match (p1, p2) {
                (Win, Rock) => Paper,
                (Win, Paper) => Scissor,
                (Win, Scissor) => Rock,
                (Lose, Rock) => Scissor,
                (Lose, Paper) => Rock,
                (Lose, Scissor) => Paper,
                (Draw, p2) => p2,
            };
            p1.round_score(p2)
        })
        .sum())
    .map(|v: i64| v.to_string())
}
