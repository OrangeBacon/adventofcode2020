use anyhow::Result;
use libaoc::{aoc, AocResult};
use std::time::Instant;

#[aoc("", "")]
pub fn solve(input: String) -> Result<AocResult> {
    let parse = Instant::now();

    let parse = parse.elapsed().as_secs_f64();
    let t1 = Instant::now();

    let t1 = t1.elapsed().as_secs_f64();

    let t2 = Instant::now();

    let t2 = t2.elapsed().as_secs_f64();

    Ok(AocResult::new("", "", parse, t1, t2))
}
