use anyhow::Result;
use libaoc::{aoc, AocResult};
use std::time::Instant;
use std::rc::Rc;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Postion {
    Floor,
    Empty,
    Occupied,
}
use Postion::*;

fn get_seat(input: &Vec<Vec<Postion>>, x: usize, y: usize, x_off: i32, y_off: i32, any_dist: bool) -> bool {
    let mut current_x = (x as i32) + x_off;
    let mut current_y = (y as i32) + y_off;

    let h = (input.len()-1) as i32;
    let w = (input[0].len()-1) as i32;

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

fn advance(input: Vec<Vec<Postion>>, any_dist: bool, occ_count: i32) -> Vec<Vec<Postion>> {
    let mut result = input.clone();
    for (y, row) in input.iter().enumerate() {
        for (x, &seat) in row.iter().enumerate() {
            if seat == Floor {
                result[y][x] = Floor;
            } else {
                let adj = vec![
                    get_seat(&input, x, y, -1, -1, any_dist),
                    get_seat(&input, x, y, -1,  0, any_dist),
                    get_seat(&input, x, y, -1,  1, any_dist),
                    get_seat(&input, x, y,  0, -1, any_dist),
                    get_seat(&input, x, y,  0,  1, any_dist),
                    get_seat(&input, x, y,  1, -1, any_dist),
                    get_seat(&input, x, y,  1,  0, any_dist),
                    get_seat(&input, x, y,  1,  1, any_dist),
                ];
                let count = adj.iter().fold(0, |a,&x|if x{a+1}else{a});

                if seat == Empty && count == 0 {
                    result[y][x] = Occupied;
                } else if seat == Occupied && count >= occ_count {
                    result[y][x] = Empty;
                } else {
                    result[y][x] = seat;
                }
            }
        }
    }
    result
}

fn iter_advance(initial_seats: &Vec<Vec<Postion>>, any_dist: bool, occ_count: i32) -> i32 {
    let mut seats = Rc::new(initial_seats.to_vec());
    let mut prev_count = 0;
    loop {
        let new = Rc::new(advance(seats.to_vec(), any_dist, occ_count));
        let mut count = 0;
        for row in &*new {
            for &seat in row {
                if seat == Occupied {count += 1}
            }
        }
        if prev_count == count {break;}
        prev_count = count;
        seats = new;
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
