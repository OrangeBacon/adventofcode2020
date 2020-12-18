use anyhow::Result;
use hashbrown::HashMap;
use itertools::Itertools;
use libaoc::{aoc, AocResult, Timer};
use std::convert::TryInto;

type Coords<const SIZE: usize> = [i32; SIZE];

fn get_neighbors<const SIZE: usize>(
    location: Coords<SIZE>,
    data: &HashMap<Coords<SIZE>, bool>,
) -> i32 {
    let offsets = [[-1i32, 0, 1]; SIZE];

    let mut count = 0;
    for current in offsets.iter().multi_cartesian_product() {
        let coords: Vec<_> = current.iter().map(|x| **x).collect();
        let coords: Coords<SIZE> = coords.try_into().unwrap();

        let coords: Vec<_> = coords.iter().zip(&location).map(|(a, b)| a + b).collect();
        let coords: Coords<SIZE> = coords.try_into().unwrap();

        if location != coords && *data.get(&coords).unwrap_or(&false) {
            count += 1;
        }
    }

    count
}

fn game<const SIZE: usize>(mut data: HashMap<[i32; SIZE], bool>, width: i32, height: i32) -> u32 {
    let mut min_pos = [0; SIZE];
    let mut size = [1; SIZE];
    size[0] = width;
    size[1] = height;

    let mut new = HashMap::new();
    for _ in 0..6 {
        for i in min_pos.iter_mut() {
            *i -= 1;
        }

        for i in size.iter_mut() {
            *i += 1;
        }

        let iterators: Vec<Vec<_>> = min_pos
            .iter()
            .zip(&size)
            .map(|(&a, &b)| (a..b).into_iter().collect())
            .collect();
        for coords in iterators.iter().multi_cartesian_product() {
            let coords: Vec<_> = coords.iter().map(|x| **x).collect();
            let coords: Coords<SIZE> = coords.try_into().unwrap();

            let current_state = *data.get(&coords).unwrap_or(&false);
            let neighbors = get_neighbors(coords, &data);
            let new_state = if current_state {
                neighbors == 2 || neighbors == 3
            } else {
                neighbors == 3
            };

            new.insert(coords, new_state);
        }

        (data, new) = (new, data);
        new.clear();
    }

    let mut count = 0;
    for &value in data.values() {
        if value {
            count += 1;
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

    let mut data_3 = HashMap::new();
    let mut data_4 = HashMap::new();
    for (y, row) in input.iter().enumerate() {
        for (x, &item) in row.iter().enumerate() {
            data_3.insert([x as i32, y as i32, 0], item);
            data_4.insert([x as i32, y as i32, 0, 0], item);
        }
    }
    timer.lap("Construct maps");

    let width = input[0].len() as i32;
    let height = input.len() as i32;

    let part1 = game(data_3, width, height);
    timer.lap("Part 1");

    let part2 = game(data_4, width, height);
    timer.lap("Part 2");

    Ok(AocResult::new(part1, part2))
}
