use anyhow::Result;
use libaoc::{aoc, AocResult, Timer};
use std::cmp::max;

#[aoc("953", "615")]
pub fn solve(timer: &mut Timer, input: &str) -> Result<AocResult> {
    let lines: Vec<_> = input.lines().collect();
    let mut seats = [[false; 8]; 128];
    timer.lap("Parse");

    let part1 = lines.iter().fold(0, |acc, &line| {
        let mut row = 0;
        let mut col = 0;
        let mut row_size = 64;
        let mut col_size = 4;
        for c in line.chars() {
            match c {
                'F' => row_size /= 2,
                'B' => {
                    // upper
                    row += row_size;
                    row_size /= 2;
                }
                'R' => {
                    // upper
                    col += col_size;
                    col_size /= 2;
                }
                'L' => col_size /= 2,
                _ => panic!(),
            }
        }
        seats[row][col] = true;
        max(acc, row * 8 + col)
    });
    timer.lap("Part 1");

    let mut found = false;
    let mut part2 = 0;
    for (y, row) in seats.iter().enumerate() {
        if !found {
            found = row.iter().fold(false, |a, x| a | x);
        } else {
            let res = row
                .iter()
                .enumerate()
                .fold(0, |_, (x, v)| if *v { 0 } else { y * 8 + x });
            if res > 0 {
                part2 = res;
                break;
            }
        }
    }
    timer.lap("Part 2");

    Ok(AocResult::new(part1, part2))
}
