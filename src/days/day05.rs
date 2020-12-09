use anyhow::Result;
use libaoc::{aoc, time, AocResult};
use std::cmp::max;
use std::time::Instant;

#[aoc("953", "615")]
pub fn day05(input: String) -> Result<AocResult> {
    let parse = Instant::now();
    let lines: Vec<_> = input.lines().collect();

    let mut seats = [[false; 8]; 128];
    let parse = parse.elapsed().as_secs_f64();

    let (part1, t1) = time(|| {
        lines.iter().fold(0, |acc, &line| {
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
        })
    });

    let t2 = Instant::now();
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
    let t2 = t2.elapsed().as_secs_f64();

    Ok(AocResult::new(part1, part2, parse, t1, t2))
}
