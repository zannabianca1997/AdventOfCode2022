#![feature(is_some_and)]
#![feature(slice_group_by)]
#![feature(int_roundings)]

extern crate clap;
extern crate regex;
#[macro_use]
extern crate lazy_static;
extern crate arrayvec;

use clap::{builder::PossibleValue, Parser, ValueEnum};
use days::{PuzzleResult, SolveFn, DAYS};
use regex::Regex;
use std::{
    error::Error,
    ffi::OsString,
    fs,
    path::{Path, PathBuf},
    time::{Duration, Instant},
};

mod days;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PartSpec {
    First,
    Second,
    Both,
}

impl ValueEnum for PartSpec {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::First, Self::Second, Self::Both]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            PartSpec::First => PossibleValue::new("first").alias("1"),
            PartSpec::Second => PossibleValue::new("second").alias("2"),
            PartSpec::Both => PossibleValue::new("both"),
        })
    }
}

/// Execute Advent of Code problems
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Inputs directory
    #[arg(long)]
    inputs_dir: Option<PathBuf>,

    /// Alternative input file name
    #[arg(long)]
    input: Option<OsString>,

    /// Day to solve (1-25) [default all]
    #[arg(long, short)]
    day: Option<u8>,

    /// Part to solve
    #[arg(short, long, default_value = "both")]
    part: PartSpec,

    /// Run visualizations
    #[arg(short, long)]
    visualize: Vec<Regex>,
}

#[derive(Debug, Clone)]
struct RunResult {
    res: PuzzleResult,
    time: Duration,
}

#[derive(Debug, Clone, Copy)]
struct RunSetup {}

#[derive(Debug, Clone)]
struct DayResult {
    p1: Option<RunResult>,
    p2: Option<RunResult>,
}

fn run_part(part: SolveFn, input: &str) -> Result<RunResult, Box<dyn Error>> {
    let start = Instant::now();
    let res = part(input);
    let time = start.elapsed();
    Ok(RunResult { res: res?, time })
}

fn run_day(day: u8, parts: PartSpec, input: &str) -> Result<DayResult, Box<dyn Error>> {
    use PartSpec::*;
    match (parts, &DAYS[day as usize - 1]) {
        (First, (Some(p1), _, _)) => Ok(DayResult {
            p1: Some(run_part(*p1, input)?),
            p2: None,
        }),
        (Second, (_, Some(p2), _)) => Ok(DayResult {
            p1: None,
            p2: Some(run_part(*p2, input)?),
        }),
        (Both, (Some(p1), Some(p2), _)) => Ok(DayResult {
            p1: Some(run_part(*p1, input)?),
            p2: Some(run_part(*p2, input)?),
        }),
        // Errors
        (First, (None, _, _)) => Err(format!("First part of day {day} is unsolved").into()),
        (Second, (_, None, _)) => Err(format!("Second part of day {day} is unsolved").into()),
        (Both, (None, _, _) | (_, None, _)) => {
            Err(format!("Some parts of day {day} are unsolved").into())
        }
    }
}

fn parse_args() -> Result<Args, Box<dyn Error>> {
    let args = Args::parse();
    // checking only one file is given
    if !(args.inputs_dir.is_some() != args.input.is_some()) {
        return Err("Only one between inputs dir and alternative input can be given".into());
    }
    // checking all info are given
    match args.day {
        Some(day) => {
            if !(1 <= day && day <= 25) {
                return Err("Day should be between 1 and 25".into());
            }
        }
        None => {
            if let Some(_) = args.input {
                return Err("Cannot given alternate input when running all days".into());
            }
            if args.part != PartSpec::Both {
                return Err("Cannot run single parts of all days".into());
            }
        }
    }
    Ok(args)
}

fn get_input_from_input_dir(day: u8, dir: &Path) -> PathBuf {
    let mut dir = dir.to_path_buf();
    dir.push(day.to_string());
    dir.push("input");
    dir
}

fn read_file(path: &Path) -> Result<String, Box<dyn Error>> {
    fs::read_to_string(path).map_err(|err| {
        format!(
            "Error while accessing file {}: {}",
            path.to_string_lossy(),
            err.to_string()
        )
        .into()
    })
}

/// Center a string
fn pad_center(s: &str, len: usize) -> String {
    if s.len() >= len {
        String::from(s)
    } else {
        let pad = (len - s.len()) / 2;
        let mut res = String::with_capacity(len);
        for _ in 0..pad {
            res.push(' ')
        }
        res.push_str(s);
        while res.len() < len {
            res.push(' ')
        }
        res
    }
}
/// left pad a string
fn pad_left(s: &str, len: usize) -> String {
    if s.len() >= len {
        String::from(s)
    } else {
        let pad = len - s.len();
        let mut res = String::with_capacity(len);
        for _ in 0..pad {
            res.push(' ')
        }
        res.push_str(s);
        res
    }
}
///create a string with a repeated char
fn repeat_char(ch: char, len: usize) -> String {
    let mut s = String::with_capacity(ch.len_utf8() * len);
    for _ in 0..len {
        s.push(ch)
    }
    s
}

/// Format a duration
fn format_duration(d: Duration) -> String {
    // humantime::format_duration(d).to_string()
    if d.as_secs() > 60 {
        // writing it as {}m {}s
        format!("{}m {:2}s", d.as_secs() / 60, d.as_secs() % 60)
    } else if d.as_secs() > 0 {
        // writing it as {}s {}ms
        format!("{}s {:3}ms", d.as_secs(), d.subsec_millis())
    } else if d.as_millis() > 0 {
        // writing it as {}ms {}us
        format!("{}ms {:3}us", d.as_millis(), d.subsec_micros() % 1000)
    } else if d.as_micros() > 0 {
        // writing it as {}us {}ns
        format!("{}us {:3}ns", d.as_micros(), d.subsec_nanos() % 1000)
    } else {
        // writing it as {}ns
        format!("{}ns", d.as_nanos())
    }
}

fn result_table(results: Vec<DayResult>) -> String {
    let header = ("day", "part one", "part two");
    let mut table = Vec::with_capacity(results.len());
    let mut multilines = vec![];

    let mut part_entry = |day: usize, part: usize, res: &RunResult| -> (String, String) {
        (
            match res.res.repr() {
                days::ResultRepr::Short(s) => s,
                days::ResultRepr::Multiline(s) => {
                    multilines.push((day, part, s));
                    "<multiline>".to_owned()
                }
            },
            format_duration(res.time).to_string(),
        )
    };

    for (d, res) in results.iter().enumerate() {
        // erase empty lines
        if res.p1.is_some() || res.p2.is_some() {
            table.push((
                (d + 1).to_string(),
                res.p1.as_ref().map(|v| part_entry(d + 1, 1, v)),
                res.p2.as_ref().map(|v| part_entry(d + 1, 2, v)),
            ))
        }
    }

    // measuring column sizes
    let day_col_size = {
        let mut size = header.0.len();
        for (s, _, _) in table.iter() {
            size = size.max(s.len())
        }
        size
    };
    let part1_res_col_size = {
        let mut size = 0;
        for (_, p, _) in table.iter() {
            if let Some((s, _)) = p {
                size = size.max(s.len())
            }
        }
        size
    };
    let part1_time_col_size = {
        let mut size = 0;
        for (_, p, _) in table.iter() {
            if let Some((_, s)) = p {
                size = size.max(s.len())
            }
        }
        size
    };
    let part2_res_col_size = {
        let mut size = 0;
        for (_, _, p) in table.iter() {
            if let Some((s, _)) = p {
                size = size.max(s.len())
            }
        }
        size
    };
    let part2_time_col_size = {
        let mut size = 0;
        for (_, _, p) in table.iter() {
            if let Some((_, s)) = p {
                size = size.max(s.len())
            }
        }
        size
    };

    // measuring complete column size
    let day_col_size = day_col_size + 2;
    let part1_col_size =
        part1_res_col_size + " (time: ".len() + part1_time_col_size + ")".len() + 2;
    let part2_col_size =
        part2_res_col_size + " (time: ".len() + part2_time_col_size + ")".len() + 2;

    // preparing header
    let hline = [
        "+",
        &repeat_char('-', day_col_size),
        "+",
        &repeat_char('-', part1_col_size),
        "+",
        &repeat_char('-', part2_col_size),
        "+\n",
    ]
    .concat();

    // build the table
    let mut table_str = String::new();
    table_str.push_str(&hline);
    table_str.push_str(
        &[
            "|",
            &pad_center(header.0, day_col_size),
            "|",
            &pad_center(header.1, part1_col_size),
            "|",
            &pad_center(header.2, part2_col_size),
            "|\n",
        ]
        .concat(),
    );
    table_str.push_str(&hline);
    for (d, p1, p2) in table {
        table_str.push_str(
            &[
                "| ",
                &pad_left(&d, day_col_size - 2),
                " |",
                &p1.map(|(r, t)| {
                    [
                        " ",
                        &pad_left(&r, part1_res_col_size),
                        " (time: ",
                        &pad_left(&t, part1_time_col_size),
                        ") ",
                    ]
                    .concat()
                })
                .unwrap_or(repeat_char(' ', part1_col_size)),
                "|",
                &p2.map(|(r, t)| {
                    [
                        " ",
                        &pad_left(&r, part2_res_col_size),
                        " (time: ",
                        &pad_left(&t, part2_time_col_size),
                        ") ",
                    ]
                    .concat()
                })
                .unwrap_or(repeat_char(' ', part2_col_size)),
                "|\n",
            ]
            .concat(),
        )
    }
    table_str.push_str(&hline);

    // adding sections
    let mut sections = vec![];
    for (day, part, result) in multilines {
        sections.push(format!(" === Day {day} part {part} ===\n\n{result}"))
    }

    // counting total time
    let (p1_total, p2_total) =
        results
            .iter()
            .fold((Duration::ZERO, Duration::ZERO), |(t1, t2), r| {
                (
                    t1 + r.p1.as_ref().map_or(Duration::ZERO, |p| p.time),
                    t2 + r.p2.as_ref().map_or(Duration::ZERO, |p| p.time),
                )
            });
    let time_totals = format!(
        "Part 1: {}\nPart 2: {}\nTotal : {}\n",
        format_duration(p1_total),
        format_duration(p2_total),
        format_duration(p1_total + p2_total)
    );

    // building the result
    let mut result = vec![table_str];
    result.extend(sections);
    result.push(time_totals);
    result.join("\n")
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = parse_args()?;

    let to_run = {
        // deciding what to run
        let mut to_run = vec![None; 25];
        match args.day {
            Some(d) => {
                to_run[d as usize - 1] = Some((
                    args.part,
                    read_file(
                        match args.input {
                            Some(alternate_file) => PathBuf::from(alternate_file),
                            None => get_input_from_input_dir(d, &args.inputs_dir.unwrap()),
                        }
                        .as_path(),
                    )?,
                ))
            }
            None => {
                let path = args.inputs_dir.unwrap();
                for d in 1..=25 {
                    use PartSpec::*;
                    let input = get_input_from_input_dir(d, &path);
                    to_run[d as usize - 1] = match DAYS[d as usize - 1] {
                        (None, None, _) => None,
                        (Some(_), None, _) => Some((First, read_file(input.as_path())?)),
                        (None, Some(_), _) => Some((Second, read_file(input.as_path())?)),
                        (Some(_), Some(_), _) => Some((Both, read_file(input.as_path())?)),
                    }
                }
            }
        }
        to_run
    };

    let results: Vec<_> =
        Result::from_iter(to_run.into_iter().enumerate().map(|(d, run)| match run {
            Some((parts, input)) => run_day(d as u8 + 1, parts, &input),
            None => Ok(DayResult { p1: None, p2: None }),
        }))?;

    print!("{}", result_table(results));

    Ok(())
}
