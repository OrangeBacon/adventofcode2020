use anyhow::Result;
use itertools::Itertools;
use libaoc::{aoc, AocResult};
use std::time::Instant;

#[aoc("3199139634", "438559930")]
pub fn solve(input: String) -> Result<AocResult> {
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
            .cartesian_product(window)
            .any(|(a, b)| a != b && a + b == test);
        if !vals {
            part1 = test;
            break;
        }
    }
    let t1 = t1.elapsed().as_secs_f64();

    let t2 = Instant::now();
    let mut left = 0;
    let mut right = 1;
    let mut sum = nums[0] + nums[1];

    let part2 = loop {
        if sum < part1 {
            right += 1;
            sum += nums[right];
        } else if sum > part1 {
            sum -= nums[left];
            left += 1;
        } else {
            let window = &nums[left..=right];
            break window.iter().max().unwrap() + window.iter().min().unwrap();
        }
    };

    let t2 = t2.elapsed().as_secs_f64();

    Ok(AocResult::new(part1, part2, parse, t1, t2))
}
