use anyhow::Result;
use hashbrown::HashMap;
use libaoc::{aoc, AocResult, Timer};
use regex::Regex;
use std::cmp::{max, min};

#[derive(Debug)]
enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

#[aoc("266", "3627")]
pub fn solve(timer: &mut Timer, input: &str) -> Result<AocResult> {
    let re = Regex::new(r"(e|se|sw|w|nw)")?;

    let lines: Vec<_> = input.lines().map(|x| re.replace_all(x, "$1,")).collect();
    let mut input: Vec<String> = vec![];
    for line in lines {
        input.push(line.to_string())
    }
    let input: Vec<Vec<_>> = input
        .iter()
        .map(|x| {
            x.split(',')
                .filter(|x| !x.is_empty())
                .map(|x| {
                    use Direction::*;
                    match x {
                        "e" => East,
                        "se" => SouthEast,
                        "sw" => SouthWest,
                        "w" => West,
                        "nw" => NorthWest,
                        "ne" => NorthEast,
                        _ => panic!(),
                    }
                })
                .collect()
        })
        .collect();

    timer.lap("Parse");

    /*
    (0,0)     (2,0)     (4,0)
         (1,1)     (3,1)
    (0,2)     (2,2)     (4,2)
         (1,3)     (3,3)
    (0,4)     (2,4)     (4,4)
    */

    // black = false, white = true

    let mut coord_set = HashMap::new();

    let mut min_x = 0;
    let mut min_y = 0;
    let mut max_x = 0;
    let mut max_y = 0;

    for line in input {
        let mut x = 0;
        let mut y = 0;
        for coord in &line {
            use Direction::*;
            match coord {
                East => x += 2,
                SouthEast => {
                    y += 1;
                    x += 1
                }
                SouthWest => {
                    y += 1;
                    x -= 1
                }
                West => x -= 2,
                NorthWest => {
                    y -= 1;
                    x -= 1
                }
                NorthEast => {
                    y -= 1;
                    x += 1
                }
            }
        }

        coord_set.insert((x, y), coord_set.contains_key(&(x, y)));

        min_x = min(min_x, x);
        min_y = min(min_y, y);
        max_x = max(max_x, x);
        max_y = max(max_y, y);
    }

    let part1 = coord_set.values().filter(|x| !**x).count();

    timer.lap("Part 1");

    let mut new_tiles = HashMap::new();
    for _ in 0..100 {
        min_x -= 1;
        min_y -= 1;
        max_x += 1;
        max_y += 1;

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                let current = *coord_set.get(&(x, y)).unwrap_or(&true);
                let adjacent = [
                    *coord_set.get(&(x + 2, y)).unwrap_or(&true),
                    *coord_set.get(&(x + 1, y + 1)).unwrap_or(&true),
                    *coord_set.get(&(x - 1, y + 1)).unwrap_or(&true),
                    *coord_set.get(&(x - 2, y)).unwrap_or(&true),
                    *coord_set.get(&(x - 1, y - 1)).unwrap_or(&true),
                    *coord_set.get(&(x + 1, y - 1)).unwrap_or(&true),
                ];
                let black_tiles = adjacent.iter().filter(|x| !**x).count();
                if current && black_tiles == 2 {
                    new_tiles.insert((x, y), false);
                } else if !current && (black_tiles == 0 || black_tiles > 2) {
                    new_tiles.insert((x, y), true);
                } else {
                    new_tiles.insert((x, y), current);
                }
            }
        }

        (coord_set, new_tiles) = (new_tiles, coord_set);
        new_tiles.clear();
    }

    let part2 = coord_set.values().filter(|x| !**x).count();

    timer.lap("Part 2");

    Ok(AocResult::new(part1, part2))
}
