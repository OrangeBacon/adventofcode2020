use anyhow::Result;
use libaoc::{aoc, AocResult, Timer};
use std::collections::BTreeMap;

fn get_neighbors_3(coords: (i32, i32, i32), data: &BTreeMap<(i32, i32, i32), bool>) -> i32 {
    let mut count = 0;
    for x in (coords.0 - 1)..=(coords.0 + 1) {
        for y in (coords.1 - 1)..=(coords.1 + 1) {
            for z in (coords.2 - 1)..=(coords.2 + 1) {
                if ((x, y, z) != coords) && *data.get(&(x, y, z)).unwrap_or(&false) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn get_neighbors_4(coords: (i32, i32, i32, i32), data: &BTreeMap<(i32, i32, i32, i32), bool>) -> i32 {
    let mut count = 0;
    for x in (coords.0 - 1)..=(coords.0 + 1) {
        for y in (coords.1 - 1)..=(coords.1 + 1) {
            for z in (coords.2 - 1)..=(coords.2 + 1) {
                for w in (coords.3 - 1)..=(coords.3 + 1) {
                    if ((x, y, z, w) != coords) && *data.get(&(x, y, z, w)).unwrap_or(&false) {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

#[aoc("346", "1632")]
pub fn solve(timer: &mut Timer, input: &str) -> Result<AocResult> {
    let input: Vec<Vec<_>> = input
        .trim()
        .lines()
        .map(|x| {
            x.chars()
                .map(|x| match x {
                    '#' => true,
                    '.' => false,
                    _ => panic!(),
                })
                .collect()
        })
        .collect();
    timer.lap("Parse");

    let mut data_3 = BTreeMap::new();
    let mut data_4 = BTreeMap::new();
    for (y, row) in input.iter().enumerate() {
        for (x, &item) in row.iter().enumerate() {
            data_3.insert((x as i32, y as i32, 0), item);
            data_4.insert((x as i32, y as i32, 0, 0), item);
        }
    }
    timer.lap("Construct maps");

    let mut min_pos = (0, 0, 0);
    let mut size = (input[0].len() as i32, input.len() as i32, 1);

    for _ in 0..6 {
        let mut new = BTreeMap::new();

        min_pos.0 -= 1;
        min_pos.1 -= 1;
        min_pos.2 -= 1;

        size.0 += 1;
        size.1 += 1;
        size.2 += 1;

        for x in min_pos.0..size.0 {
            for y in min_pos.1..size.1 {
                for z in min_pos.2..size.2 {
                    let coords = (x, y, z);
                    let current_state = *data_3.get(&coords).unwrap_or(&false);
                    let neighbors = get_neighbors_3(coords, &data_3);
                    let new_state = if current_state {
                        if neighbors == 2 || neighbors == 3 {
                            true
                        } else {
                            false
                        }
                    } else {
                        if neighbors == 3 {
                            true
                        } else {
                            false
                        }
                    };

                    new.insert(coords, new_state);
                }
            }
        }

        data_3 = new;
    }

    let mut count = 0;
    for &value in data_3.values() {
        if value {
            count += 1;
        }
    }
    let part1 = count;

    timer.lap("Part 1");


    let mut min_pos = (0, 0, 0, 0);
    let mut size = (input[0].len() as i32, input.len() as i32, 1, 1);

    for _ in 0..6 {
        let mut new = BTreeMap::new();

        min_pos.0 -= 1;
        min_pos.1 -= 1;
        min_pos.2 -= 1;
        min_pos.3 -= 1;

        size.0 += 1;
        size.1 += 1;
        size.2 += 1;
        size.3 += 1;

        for x in min_pos.0..size.0 {
            for y in min_pos.1..size.1 {
                for z in min_pos.2..size.2 {
                    for w in min_pos.3..size.3 {
                        let coords = (x, y, z, w);
                        let current_state = *data_4.get(&coords).unwrap_or(&false);
                        let neighbors = get_neighbors_4(coords, &data_4);
                        let new_state = if current_state {
                            if neighbors == 2 || neighbors == 3 {
                                true
                            } else {
                                false
                            }
                        } else {
                            if neighbors == 3 {
                                true
                            } else {
                                false
                            }
                        };

                        new.insert(coords, new_state);
                    }
                }
            }
        }

        data_4 = new;
    }

    let mut count = 0;
    for &value in data_4.values() {
        if value {
            count += 1;
        }
    }
    let part2 = count;
    timer.lap("Part 2");

    Ok(AocResult::new(part1, part2))
}
