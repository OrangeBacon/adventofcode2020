use anyhow::Result;
use libaoc::{aoc, AocResult, Timer};

fn iter(lines: &[Vec<char>], depth: usize, height: usize) -> i32 {
    let mut x = 0;
    let mut trees = 0;
    for (i, line) in lines.iter().enumerate() {
        if i % height != 0 {
            continue;
        }
        if line[x] == '#' {
            trees += 1;
        }
        x = (x + depth) % line.len();
    }
    trees
}

#[aoc("254", "1666768320")]
pub fn solve(timer: &mut Timer, input: &str) -> Result<AocResult> {
    let lines = input
        .lines()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    timer.lap("Parse");

    let part1 = iter(&lines, 3, 1);
    timer.lap("Part 1");

    let part2 =
        iter(&lines, 1, 1) * part1 * iter(&lines, 5, 1) * iter(&lines, 7, 1) * iter(&lines, 1, 2);
    timer.lap("Part 2");

    Ok(AocResult::new(part1, part2))
}
