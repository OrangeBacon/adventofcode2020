use crate::Solution;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;

/// function to run each day's problem
pub const SOLUTIONS: &[Solution] = &[
    day01::day01,
    day02::day02,
    day03::day03,
    day04::day04,
    day05::day05,
    day06::day06,
    day07::day07,
    day08::day08,
    day09::day09,
];

/// list of all default test data
pub const DEFAULT_DATA: &[&str] = &[
    include_str!("../../data/day1.txt"),
    include_str!("../../data/day2.txt"),
    include_str!("../../data/day3.txt"),
    include_str!("../../data/day4.txt"),
    include_str!("../../data/day5.txt"),
    include_str!("../../data/day6.txt"),
    include_str!("../../data/day7.txt"),
    include_str!("../../data/day8.txt"),
    include_str!("../../data/day9.txt"),
];
