use anyhow::Result;
use libaoc::{aoc, AocResult};
use std::time::Instant;

enum Movement {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}
use Movement::*;

impl Movement {
    fn new(name: char) -> Self {
        match name {
            'N' => North,
            'S' => South,
            'E' => East,
            'W' => West,
            'L' => Left,
            'R' => Right,
            'F' => Forward,
            _ => panic!(),
        }
    }
}

fn rotate_one(current: (i32, i32)) -> (i32, i32) {
    match current {
        (1, 0) => (0, 1),
        (0, 1) => (-1, 0),
        (-1, 0) => (0, -1),
        (0, -1) => (1, 0),
        _ => panic!(),
    }
}

fn rotate(current: (i32, i32), left: bool, amount: i32) -> (i32, i32) {
    let amount = if left { amount } else { 4 - amount };

    let mut res = current;
    for _ in 0..amount {
        res = rotate_one(res);
    }
    res
}

fn waypoint((x, y): (i32, i32), left: bool, amount: i32) -> (i32, i32) {
    let direction = rotate((1, 0), left, amount);
    match direction {
        (0, 1) => (-y, x),
        (-1, 0) => (-x, -y),
        (0, -1) => (y, -x),
        _ => panic!(),
    }
}

fn run(input: &[(Movement, i32)], is_waypoint: bool) -> i32 {
    let mut pos = (0, 0);
    let mut point = (1, 0);

    let func = if is_waypoint { waypoint } else { rotate };

    for (code, count) in input {
        match code {
            North if is_waypoint => point.1 += count,
            South if is_waypoint => point.1 -= count,
            East if is_waypoint => point.0 += count,
            West if is_waypoint => point.0 -= count,
            North => pos.1 += count,
            South => pos.1 -= count,
            East => pos.0 += count,
            West => pos.0 -= count,
            Left => point = func(point, true, count / 90),
            Right => point = func(point, false, count / 90),
            Forward => {
                pos.0 += count * point.0;
                pos.1 += count * point.1;
            }
        }
    }
    pos.0.abs() + pos.1.abs()
}

#[aoc("2297", "89984")]
pub fn solve(input: String) -> Result<AocResult> {
    let parse = Instant::now();
    let input: Vec<_> = input
        .trim()
        .lines()
        .map(|x| {
            (
                Movement::new(x.chars().nth(0).unwrap()),
                x[1..].parse::<i32>().unwrap(),
            )
        })
        .collect();
    let parse = parse.elapsed().as_secs_f64();

    let t1 = Instant::now();
    let part1 = run(&input, false);
    let t1 = t1.elapsed().as_secs_f64();

    let t2 = Instant::now();
    let part2 = run(&input, true);
    let t2 = t2.elapsed().as_secs_f64();

    Ok(AocResult::new(part1, part2, parse, t1, t2))
}
