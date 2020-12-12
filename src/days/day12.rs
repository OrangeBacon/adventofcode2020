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

fn waypoint(left: bool, amount: i32, x: i32, y: i32) -> (i32, i32) {
    let direction = rotate((1, 0), left, amount);
    match direction {
        (0, 1) => (-y, x),
        (-1, 0) => (-x, -y),
        (0, -1) => (y, -x),
        _ => panic!(),
    }
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
    let mut x = 0;
    let mut y = 0;
    let mut direction = (1, 0);
    for (code, count) in &input {
        match code {
            North => y += count,
            South => y -= count,
            East => x += count,
            West => x -= count,
            Left => direction = rotate(direction, true, count / 90),
            Right => direction = rotate(direction, false, count / 90),
            Forward => {
                x += count * direction.0;
                y += count * direction.1;
            }
        }
    }
    let part1 = x.abs() + y.abs();
    let t1 = t1.elapsed().as_secs_f64();

    let t2 = Instant::now();
    let mut x = 0;
    let mut y = 0;
    let mut waypoint_x = 10;
    let mut waypoint_y = 1;
    for (code, count) in input {
        match code {
            North => waypoint_y += count,
            South => waypoint_y -= count,
            East => waypoint_x += count,
            West => waypoint_x -= count,
            Left => (waypoint_x, waypoint_y) = waypoint(true, count / 90, waypoint_x, waypoint_y),
            Right => (waypoint_x, waypoint_y) = waypoint(false, count / 90, waypoint_x, waypoint_y),
            Forward => {
                x += waypoint_x * count;
                y += waypoint_y * count;
            }
        }
    }
    let part2 = x.abs() + y.abs();
    let t2 = t2.elapsed().as_secs_f64();

    Ok(AocResult::new(part1, part2, parse, t1, t2))
}
