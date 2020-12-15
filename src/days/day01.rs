use anyhow::Result;
use libaoc::{aoc, AocResult, Timer};

#[aoc("964875", "158661360")]
pub fn solve(timer: &mut Timer, input: &str) -> Result<AocResult> {
    let nums = input
        .split_whitespace()
        .map(|x| x.parse::<i32>())
        .collect::<Result<Vec<_>, _>>()?;

    timer.lap("Parse");

    let mut part1 = 0;
    'out: for (i, num) in nums.iter().enumerate() {
        for second in nums[(i + 1)..].iter() {
            if num + second == 2020 {
                part1 = num * second;
                break 'out;
            }
        }
    }
    timer.lap("Part 1");

    let mut part2 = 0;
    'out2: for (i, num) in nums.iter().enumerate() {
        for (j, second) in nums[(i + 1)..].iter().enumerate() {
            for third in nums[(j + 1)..].iter() {
                if num + second + third == 2020 {
                    part2 = num * second * third;
                    break 'out2;
                }
            }
        }
    }
    timer.lap("Part 2");

    Ok(AocResult::new(part1, part2))
}
