use anyhow::Result;
use libaoc::{aoc, AocResult};
use std::time::Instant;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Postion {
    Floor,
    Empty,
    Occupied,
}
use Postion::*;

fn get_seat(input: &Vec<Vec<Postion>>, w: i32, h:i32, x: i32, y: i32, x_off: i32, y_off: i32, any_dist: bool) -> bool {
    let mut current_x = x + x_off;
    let mut current_y = y + y_off;

    let mut i = 0;
    let mut result = Empty;
    loop {
        if i > 0 && !any_dist {break;}
        if current_x < 0 || current_x > w {break;}
        if current_y < 0 || current_y > h {break;}
        let tile = input[current_y as usize][current_x as usize];
        if tile != Floor {
            result = tile;
            break;
        }
        current_x += x_off;
        current_y += y_off;
        i += 1;
    }
    result == Occupied
}

fn advance(input: &Vec<Vec<Postion>>, output: &mut Vec<Vec<Postion>>, any_dist: bool, occ_count: i32) -> i32 {
    let h = (input.len()-1) as i32;
    let w = (input[0].len()-1) as i32;

    let mut seats_occupied = 0;
    for (y, row) in input.iter().enumerate() {
        'next: for (x, &seat) in row.iter().enumerate() {
            if seat == Floor {
                output[y][x] = Floor;
            } else {
                const OFFSETS: [(i32,i32);8] = [
                    (-1, -1),
                    (-1,  0),
                    (-1,  1),
                    (0, -1),
                    (0,  1),
                    (1, -1),
                    (1,  0),
                    (1,  1),
                ];
                let mut count = 0;
                for off in &OFFSETS {
                    count += if get_seat(&input, w, h, x as i32, y as i32, off.0, off.1, any_dist) {1} else {0};
                    if seat == Occupied && count >= occ_count {
                        output[y][x] = Empty; continue 'next;
                    }
                }

                if seat == Empty && count == 0 {
                    output[y][x] = Occupied;
                    seats_occupied += 1;
                } else {
                    output[y][x] = seat;
                    if seat == Occupied {seats_occupied += 1}
                }
            }
        }
    }
    seats_occupied
}

fn iter_advance(initial_seats: &Vec<Vec<Postion>>, any_dist: bool, occ_count: i32) -> i32 {
    let mut seats1 = initial_seats.to_vec().clone();
    let mut seats2 = initial_seats.to_vec();
    let mut prev_count = 0;
    loop {
        let count = advance(&seats1, &mut seats2, any_dist, occ_count);
        if prev_count == count {break;}
        prev_count = count;
        (seats1, seats2) = (seats2, seats1);
    }
    prev_count
}

#[aoc("2494", "2306")]
pub fn solve(input: String) -> Result<AocResult> {
    let parse = Instant::now();
    let initial_seats: Vec<Vec<_>> = input.lines().map(|x|x.trim().chars().map(|x| {
        match x {'L'=>Empty,'.'=>Floor,_=>panic!()}
    }).collect()).collect();
    let parse = parse.elapsed().as_secs_f64();

    let t1 = Instant::now();
    let part1 = iter_advance(&initial_seats, false, 4);
    let t1 = t1.elapsed().as_secs_f64();

    let t2 = Instant::now();
    let part2 = iter_advance(&initial_seats, true, 5);
    let t2 = t2.elapsed().as_secs_f64();

    Ok(AocResult::new(part1, part2, parse, t1, t2))
}
