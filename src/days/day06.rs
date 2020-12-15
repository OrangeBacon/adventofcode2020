use anyhow::Result;
use hashbrown::HashSet;
use libaoc::{aoc, AocResult, Timer};
use regex::Regex;

#[aoc("7283", "3520")]
pub fn solve(timer: &mut Timer, input: String) -> Result<AocResult> {
    let line = Regex::new(r"(\r?\n){2}")?;

    let chars1 = line
        .split(&input)
        .map(|x| x.chars().filter(|&x| x != '\n' && x != '\r'));
    timer.lap("Parse part 1");

    let chars2 = line.split(&input).map(|x| {
        x.lines()
            .map(|x| x.chars().collect())
            .collect::<Vec<Vec<_>>>()
    });
    timer.lap("Parse part 2");

    let part1 = chars1.fold(0, |acc, group| group.collect::<HashSet<char>>().len() + acc);
    timer.lap("Part 1");

    let part2 = chars2.fold(0, |acc, group| {
        group
            .iter()
            .map(|x| x.iter().copied().collect())
            .fold(
                group[0].iter().copied().collect(),
                |a, person: HashSet<char>| &a & &person,
            )
            .len()
            + acc
    });
    timer.lap("Part 2");

    Ok(AocResult::new(part1, part2))
}
