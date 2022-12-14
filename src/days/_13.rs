use super::PuzzleResult;
use std::{cmp::Ordering, error::Error, iter::Peekable};

#[derive(Debug, PartialEq, Eq, Clone)]
enum Item {
    Num(usize),
    List(Vec<Item>),
}
impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Item {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use Item::*;
        match (self, other) {
            (Num(n1), Num(n2)) => {
                return n1.cmp(n2);
            }
            (Num(_), List(_)) => {
                return Item::List(vec![(*self).clone()]).cmp(other);
            }
            (List(_), Num(_)) => {
                return self.cmp(&Item::List(vec![(*other).clone()]));
            }
            (List(v1), List(v2)) => {
                for idx in 0.. {
                    match (v1.get(idx), v2.get(idx)) {
                        (None, None) => return Ordering::Equal,
                        (None, Some(_)) => return Ordering::Less,
                        (Some(_), None) => return Ordering::Greater,
                        (Some(a), Some(b)) => match a.cmp(b) {
                            Ordering::Equal => (),
                            ord => {
                                return ord;
                            }
                        },
                    }
                }
            }
        }
        unreachable!()
    }
}
impl Item {
    fn try_from_chars<ChIter>(chs: &mut Peekable<ChIter>) -> Result<Self, String>
    where
        ChIter: Iterator<Item = char>,
    {
        match chs.peek() {
            Some('[') => {
                chs.next();
                if let Some(']') = chs.peek() {
                    chs.next();
                    Ok(Self::List(vec![]))
                } else {
                    let mut items = vec![Self::try_from_chars(chs)?];
                    while let Some(',') = chs.peek() {
                        chs.next();
                        items.push(Self::try_from_chars(chs)?);
                    }
                    if let Some(']') = chs.next() {
                        Ok(Self::List(items))
                    } else {
                        Err("Expected ]".to_owned())
                    }
                }
            }
            Some(ch) if ch.is_digit(10) => {
                let mut digits = String::new();
                while chs.peek().is_some_and(|ch| ch.is_digit(10)) {
                    digits.push(chs.next().unwrap())
                }
                let n = digits.parse::<usize>().map_err(|err| err.to_string())?;
                Ok(Self::Num(n))
            }
            Some(ch) => Err(format!("Unexpected char {ch:?}")),
            None => Err("Expected Items".to_owned()),
        }
    }
}
impl TryFrom<&str> for Item {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let chs = &mut value.chars().peekable();
        let res = Self::try_from_chars(chs)?;
        if let None = chs.next() {
            Ok(res)
        } else {
            Err("String not consumed".to_owned())
        }
    }
}

fn parse_input(input: &str) -> Result<Vec<(Item, Item)>, Box<dyn Error>> {
    Result::from_iter(input.trim().split("\n\n").map(|pair| {
        pair.split_once("\n")
            .ok_or("Missing newline".to_owned())
            .and_then(|(p1, p2)| {
                Item::try_from(p1).and_then(|p1| Item::try_from(p2).map(|p2| (p1, p2)))
            })
    }))
    .map_err(|err| err.into())
}

fn parse_input_2(input: &str) -> Result<Vec<Item>, Box<dyn Error>> {
    Result::from_iter(
        input
            .split("\n")
            .map(|line| line.trim())
            .filter(|line| line.len() > 0)
            .map(|line| line.try_into().map_err(|err: String| err.into())),
    )
}

pub fn part1(input: &str) -> Result<PuzzleResult, Box<dyn Error>> {
    Ok(PuzzleResult::Numeric(
        parse_input(input)?
            .into_iter()
            .enumerate()
            .filter_map(|(i, (p1, p2))| (p1 < p2).then_some(i + 1))
            .sum::<usize>() as i64,
    ))
}

pub fn part2(input: &str) -> Result<PuzzleResult, Box<dyn Error>> {
    let mut input = parse_input_2(input)?;
    let marker_2 = Item::try_from("[[2]]").unwrap();
    let marker_6 = Item::try_from("[[6]]").unwrap();
    input.push(marker_2.clone());
    input.push(marker_6.clone());
    input.sort();
    Ok(PuzzleResult::Numeric(
        ((input.binary_search(&marker_2).unwrap() + 1)
            * (input.binary_search(&marker_6).unwrap() + 1)) as i64,
    ))
}
