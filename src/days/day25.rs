use anyhow::Result;
use libaoc::{aoc, AocResult, Timer};

fn trans(subject: usize, loop_size: usize) -> usize {
    let mut value = 1;
    for _ in 0..loop_size {
        value *= subject;
        value %= 20201227;
    }
    value
}

#[aoc("15217943", "")]
pub fn solve(timer: &mut Timer, input: &str) -> Result<AocResult> {
    let input: Vec<usize> = input.lines().map(|x|x.parse().unwrap()).collect();
    timer.lap("Parse");

    let mut card_loop = 0;
    let mut value = 1;
    while value != input[0] {
        value *= 7;
        value %= 20201227;
        card_loop += 1;
    }

    let part1 = trans(input[1], card_loop);


    timer.lap("Part 1");


    timer.lap("Part 2");

    Ok(AocResult::new(part1, ""))
}
