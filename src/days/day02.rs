use anyhow::Result;
use libaoc::{aoc, AocResult, Timer};
use regex::Regex;

#[aoc("483", "482")]
pub fn solve(timer: &mut Timer, input: String) -> Result<AocResult> {
    let lines = input.lines();
    let extract = Regex::new(r"(\d+)-(\d+) (.): (.+)")?;

    timer.lap("Parse");

    let mut part1 = 0;
    let mut part2 = 0;
    for line in lines {
        let matches = extract.captures(line).unwrap();
        let min = matches.get(1).unwrap().as_str().parse::<usize>()?;
        let max = matches.get(2).unwrap().as_str().parse::<usize>()?;
        let c = matches.get(3).unwrap().as_str().chars().next().unwrap();
        let password = matches.get(4).unwrap().as_str();

        let c_count = password.matches(c).count();
        if c_count >= min && c_count <= max {
            part1 += 1;
        }

        let min_c = password.chars().nth(min - 1).unwrap();
        let max_c = password.chars().nth(max - 1).unwrap();
        if (min_c == c && max_c != c) || (min_c != c && max_c == c) {
            part2 += 1;
        }
    }

    timer.lap("Parts 1 + 2");

    Ok(AocResult::new(part1, part2))
}
