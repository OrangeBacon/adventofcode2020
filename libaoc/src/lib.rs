#![feature(str_split_once)]

use anyhow::Result;
use std::fmt;
use std::string::ToString;
use std::time::Instant;

mod vm;
pub use vm::*;

pub use aoc_attr::aoc;

/// type for each day's function, implemented by days/*.rs
pub struct Solution {
    pub run: fn(String) -> Result<AocResult>,
    pub file: &'static str,
}

impl Solution {
    pub fn run(&self, arg: String) -> Result<AocResult> {
        (self.run)(arg)
    }
}

/// generic result container for each day
pub struct AocResult {
    pub part1: String,
    pub part2: String,
    parse_time: f64,
    part1_time: f64,
    part2_time: f64,
}

impl AocResult {
    /// construct result from any valid types
    pub fn new<T: ToString, R: ToString>(part1: T, part2: R, parse: f64, t1: f64, t2: f64) -> Self {
        AocResult {
            part1: part1.to_string(),
            part2: part2.to_string(),
            parse_time: parse,
            part1_time: t1,
            part2_time: t2,
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
        writeln!(f, "  Part 2: {}", self.part2)?;
        writeln!(f, "Timing:")?;
        writeln!(f, "  Parsing: {}", FloatTime::from(self.parse_time))?;
        writeln!(f, "  Part 1: {}", FloatTime::from(self.part1_time))?;
        writeln!(f, "  Part 2: {}", FloatTime::from(self.part2_time))
    }
}

/// return the time taken to execute a function + the function's result
pub fn time<F, T>(func: F) -> (T, f64)
where
    F: FnOnce() -> T,
{
    let time = Instant::now();
    let res = func();
    let time = time.elapsed().as_secs_f64();
    (res, time)
}
