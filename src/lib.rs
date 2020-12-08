#![feature(str_split_once)]

use anyhow::Result;
use std::fmt;
use std::string::ToString;
use std::time::Instant;

pub mod days;

mod vm;
pub use vm::*;

/// type for each day's function, implemented by days/*.rs
pub type Solution = fn(String) -> Result<AocResult>;

/// generic result container for each day
pub struct AocResult {
    part1: String,
    part2: String,
    parse_time: f64,
    part1_time: f64,
    part2_time: f64,
}

impl AocResult {
    /// construct result from any valid types
    fn new<T: ToString, R: ToString>(part1: T, part2: R, parse: f64, t1: f64, t2: f64) -> Self {
        AocResult {
            part1: part1.to_string(),
            part2: part2.to_string(),
            parse_time: parse,
            part1_time: t1,
            part2_time: t2,
        }
    }
}

/// nicely format f64 as closest metric time unit
fn print_time(mut num: f64, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    let mut order = 0;

    if num == 0.0 {
        return writeln!(f, "no measured time");
    }

    while num < 1.0 {
        num *= 1000.0;
        order += 1;
    }

    writeln!(
        f,
        "{:.3}{}",
        num,
        match order {
            0 => "s",
            1 => "ms",
            2 => "μs",
            3 => "ns",
            _ => "?",
        }
    )
}

impl fmt::Display for AocResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        writeln!(f, "Results:")?;
        writeln!(f, "  Part 1: {}", self.part1)?;
        writeln!(f, "  Part 2: {}", self.part2)?;
        writeln!(f, "Timing:")?;
        write!(f, "  Parsing:")?;
        print_time(self.parse_time, f)?;
        write!(f, "  Part 1:")?;
        print_time(self.part1_time, f)?;
        write!(f, "  Part 2:")?;
        print_time(self.part2_time, f)
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
