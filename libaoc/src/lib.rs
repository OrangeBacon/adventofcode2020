#![feature(str_split_once)]

use anyhow::{anyhow, Result};
use std::{cmp::max, fmt, string::ToString};

mod vm;
pub use vm::*;

mod timer;
pub use timer::*;

pub use aoc_attr::aoc;

/// type for each day's function, implemented by days/*.rs
/// the cleanup function is optionally run, once per group of solutions with
/// the same name.  a random cleanup function will be run, so they must be
/// the same, or only have one provided in the group
pub struct Solution {
    pub number: usize,
    pub name: &'static str,
    pub run: fn(&mut Timer, &str) -> Result<AocResult>,
    pub takes_file_name: bool,
    pub cleanup_fn: Option<fn() -> ()>,
}

impl PartialEq for Solution {
    fn eq(&self, a: &Solution) -> bool {
        if self.number != a.number {
            return false;
        }
        if self.name != a.name {
            return false;
        }
        true
    }
}

impl Solution {
    pub fn run(&self, timer: &mut timer::Timer, arg: &str) -> Result<AocResult> {
        (self.run)(timer, arg)
    }

    pub fn get(
        solutions: &'static [Solution],
        day: usize,
        name: &str,
    ) -> Result<&'static Solution> {
        solutions
            .iter()
            .find(|&x| x.number == day && x.name == name)
            .ok_or_else(|| anyhow!("Could not find solution"))
    }

    pub fn latest_day(solutions: &'static [Solution]) -> usize {
        solutions
            .iter()
            .fold(0, |acc, x| std::cmp::max(acc, x.number))
    }
}

pub struct AocFile {
    pub number: usize,
    pub data: &'static str,
}

impl AocFile {
    pub fn get(files: &'static [AocFile], day: usize) -> Result<&'static str> {
        files
            .iter()
            .find(|&x| x.number == day)
            .ok_or_else(|| anyhow!("Could not find solution"))
            .map(|x| x.data)
    }
}

/// generic result container for each day
#[derive(Default)]
pub struct AocResult {
    pub results: Vec<(&'static str, String)>,
}

impl AocResult {
    /// construct result from any valid types
    pub fn new<T: ToString, R: ToString>(part1: T, part2: R) -> Self {
        AocResult {
            results: vec![("Part 1", part1.to_string()), ("Part 2", part2.to_string())],
        }
    }

    pub fn from(results: &[(&'static str, String)]) -> Self {
        AocResult {
            results: results.to_vec(),
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
        for (i, result) in self.results.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", result.1)?;
        }
        Ok(())
    }
}

impl fmt::Debug for AocResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        if !self.results.is_empty() {
            writeln!(f, "Results:")?;
        }

        let width = self
            .results
            .iter()
            .fold(0, |acc, (name, _)| max(acc, name.len()))
            + 1;

        for (name, result) in &self.results {
            writeln!(
                f,
                "  {:<width$} {}",
                format!("{}:", name),
                result,
                width = width
            )?;
        }
        Ok(())
    }
}
