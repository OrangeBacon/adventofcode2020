#![feature(str_split_once)]

use anyhow::{Error, Result};
use std::fmt;
use std::string::ToString;

mod vm;
pub use vm::*;

mod timer;
pub use timer::*;

pub use aoc_attr::aoc;

/// type for each day's function, implemented by days/*.rs
pub struct Solution {
    pub number: usize,
    pub name: &'static str,
    pub run: fn(&mut Timer, String) -> Result<AocResult>,
    pub file: &'static str,
}

impl Solution {
    pub fn run(&self, timer: &mut timer::Timer, arg: String) -> Result<AocResult> {
        (self.run)(timer, arg)
    }
}

pub fn get_solution(solutions: &'static [Solution], day: usize) -> Result<&Solution> {
    if let Some(sol) = solutions.iter().find(|&x| x.number == day) {
        Ok(sol)
    } else {
        Err(Error::msg(format!(
            "Could not find solution {}, {} solutions avaliable",
            day,
            solutions.len()
        )))
    }
}

/// generic result container for each day
pub struct AocResult {
    pub part1: String,
    pub part2: String,
}

impl AocResult {
    /// construct result from any valid types
    pub fn new<T: ToString, R: ToString>(part1: T, part2: R) -> Self {
        AocResult {
            part1: part1.to_string(),
            part2: part2.to_string(),
        }
    }
}

pub struct FloatTime {
    time: f64,
}

impl From<f64> for FloatTime {
    fn from(f: f64) -> Self {
        FloatTime { time: f }
    }
}

impl fmt::Display for FloatTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let mut order = 0;
        let mut time = self.time;

        if time == 0.0 {
            return write!(f, "no measured time");
        }

        while time < 1.0 {
            time *= 1000.0;
            order += 1;
        }

        write!(
            f,
            "{:.3}{}",
            time,
            match order {
                0 => "s",
                1 => "ms",
                2 => "μs",
                3 => "ns",
                _ => "?",
            }
        )
    }
}

impl fmt::Display for AocResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}, {}", self.part1, self.part2)
    }
}

impl fmt::Debug for AocResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        writeln!(f, "Results:")?;
        writeln!(f, "  Part 1: {}", self.part1)?;
        writeln!(f, "  Part 2: {}", self.part2)
    }
}
