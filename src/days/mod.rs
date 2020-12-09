use libaoc::Solution;

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
    day01::solution,
    day02::solution,
    day03::solution,
    day04::solution,
    day05::solution,
    day06::solution,
    day07::solution,
    day08::solution,
    day09::solution,
];
