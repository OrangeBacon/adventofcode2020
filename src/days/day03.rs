use anyhow::Result;
use libaoc::{aoc, time, AocResult};
use std::time::Instant;

fn iter(lines: &Vec<Vec<char>>, depth: usize, height: usize) -> i32 {
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

#[aoc]
pub fn day03(input: String) -> Result<AocResult> {
    let parse = Instant::now();
    let lines = input
        .lines()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let parse = parse.elapsed().as_secs_f64();

    let (part1, t1) = time(|| iter(&lines, 3, 1));

    let (part2, t2) = time(|| {
        iter(&lines, 1, 1) * part1 * iter(&lines, 5, 1) * iter(&lines, 7, 1) * iter(&lines, 1, 2)
    });

    Ok(AocResult::new(part1, part2, parse, t1, t2))
}

#[cfg(test)]
mod test {
    use crate::days::*;
    use anyhow::Result;

    #[test]
    fn day03a() -> Result<()> {
        let res = day03::day03(SOLUTIONS[2].file.to_string())?;
        assert_eq!(res.part1, "254");
        Ok(())
    }

    #[test]
    fn day03b() -> Result<()> {
        let res = day03::day03(SOLUTIONS[2].file.to_string())?;
        assert_eq!(res.part2, "1666768320");
        Ok(())
    }
}
