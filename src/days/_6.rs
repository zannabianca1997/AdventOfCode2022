use std::error::Error;

pub fn part1(input: &str) -> Result<String, Box<dyn Error>> {
    let input: Vec<_> = input.chars().collect();
    const WINDOW: usize = 4;
    'outer: for (pos, window) in input.windows(WINDOW).enumerate() {
        for i in 0..window.len() {
            for j in 0..i {
                if window[i] == window[j] {
                    continue 'outer;
                }
            }
        }
        return Ok((pos + WINDOW).to_string());
    }
    Err("Marker not found".into())
}

pub fn part2(input: &str) -> Result<String, Box<dyn Error>> {
    let input: Vec<_> = input.chars().collect();
    const WINDOW: usize = 14;
    'outer: for (pos, window) in input.windows(WINDOW).enumerate() {
        for i in 0..window.len() {
            for j in 0..i {
                if window[i] == window[j] {
                    continue 'outer;
                }
            }
        }
        return Ok((pos + WINDOW).to_string());
    }
    Err("Marker not found".into())
}
