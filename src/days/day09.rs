use anyhow::Result;
use itertools::Itertools;
use libaoc::{aoc, AocResult, Timer};
use std::cmp::Ordering;

#[aoc("3199139634", "438559930")]
pub fn solve(timer: &mut Timer, input: String) -> Result<AocResult> {
    let nums: Vec<_> = input.lines().map(|x| x.parse::<u64>().unwrap()).collect();
    timer.lap("Parse");

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
    timer.lap("Part 1");

    let mut left = 0;
    let mut right = 1;
    let mut sum = nums[0] + nums[1];

    let part2 = loop {
        match sum.cmp(&part1) {
            Ordering::Less => {
                right += 1;
                sum += nums[right];
            }
            Ordering::Greater => {
                sum -= nums[left];
                left += 1;
            }
            Ordering::Equal => {
                let window = &nums[left..=right];
                break window.iter().max().unwrap() + window.iter().min().unwrap();
            }
        }
    };
    timer.lap("Part 2");

    Ok(AocResult::new(part1, part2))
}
