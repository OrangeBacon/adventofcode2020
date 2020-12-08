#![feature(str_split_once)]

use anyhow::Result;
use std::fmt;
use std::string::ToString;

pub mod days;

pub type Solution = fn(String) -> Result<AocResult>;

pub struct AocResult {
    part1: String,
    part2: String,
}

impl AocResult {
    fn new<T: ToString, R: ToString>(part1: T, part2: R) -> Self {
        AocResult {
            part1: part1.to_string(),
            part2: part2.to_string(),
        }
    }
}

impl fmt::Display for AocResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        writeln!(f, "Part 1: {}", self.part1)?;
        writeln!(f, "Part 2: {}", self.part2)
    }
}
