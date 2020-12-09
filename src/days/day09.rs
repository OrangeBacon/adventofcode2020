use anyhow::Result;
use itertools::Itertools;
use libaoc::{aoc, AocResult};
use std::time::Instant;

#[aoc]
pub fn day09(input: String) -> Result<AocResult> {
    let parse = Instant::now();
    let nums: Vec<_> = input.lines().map(|x| x.parse::<u64>().unwrap()).collect();
    let parse = parse.elapsed().as_secs_f64();

    let t1 = Instant::now();
    let mut part1 = 0;

    let window_size = 25;
    for i in 0..(nums.len() - window_size) {
        let window = &nums[i..(i + window_size)];
        let test = nums[i + window_size];
        let vals = window
            .iter()
            .permutations(2)
            .map(|x| x[0] + x[1])
            .find(|&x| x == test);
        if vals.is_none() {
            part1 = test;
            break;
        }
    }
    let t1 = t1.elapsed().as_secs_f64();

    let t2 = Instant::now();
    let mut part2 = 0;
    'out: for window_size in 2..nums.len() {
        for i in 0..(nums.len() - window_size) {
            let window = &nums[i..(i + window_size)];
            let sum = window.iter().sum::<u64>();
            if sum == part1 {
                part2 = window.iter().max().unwrap() + window.iter().min().unwrap();
                break 'out;
            }
        }
    }
    let t2 = t2.elapsed().as_secs_f64();

    Ok(AocResult::new(part1, part2, parse, t1, t2))
}

#[cfg(test)]
mod test {
    use crate::days::*;
    use anyhow::Result;

    #[test]
    fn day09a() -> Result<()> {
        let res = day09::day09(SOLUTIONS[8].file.to_string())?;
        assert_eq!(res.part1, "3199139634");
        Ok(())
    }

    #[test]
    fn day09b() -> Result<()> {
        let res = day09::day09(SOLUTIONS[8].file.to_string())?;
        assert_eq!(res.part2, "438559930");
        Ok(())
    }
}
