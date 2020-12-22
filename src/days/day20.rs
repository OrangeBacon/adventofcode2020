use anyhow::Result;
use hashbrown::HashSet;
use libaoc::{aoc, AocResult, Timer};
use regex::Regex;
use std::cell::RefCell;
use std::collections::BTreeMap;

#[derive(Clone, Debug, Copy)]
struct Adjacency {
    id: usize,
    side: usize,
    flip: bool,
}
/*
top
right
bottom
left
*/
#[derive(Clone, Debug)]
struct Tile {
    data: Vec<Vec<char>>,
    sides: [usize; 4],
    flip_sides: [usize; 4],
    adjacency: [Vec<Adjacency>; 4],
    rotation: usize,
    flip: bool,
}

impl Tile {
    fn get_side(&self, side: usize) -> Option<usize> {
        let mut adj = self.adjacency.to_vec();
        if self.flip {
            adj.swap(0, 2);
        }
        adj.rotate_right(self.rotation);
        adj[side].get(0).map(|x|x.id)
    }

    /// applies flips and rotation until side side == id
    fn set_transform(&mut self, side: usize, id: usize, side2: usize, id2: usize) {
        let mut adj = self.adjacency.to_vec();
        for i in 0..4 {
            if ((id == 0 && adj[side].is_empty()) || (!adj[side].is_empty() && adj[side][0].id == id))
                && ((id2 == 0 && adj[side2].is_empty())
                    || (!adj[side2].is_empty() && adj[side2][0].id == id2))
            {
                self.rotation = i;
                return;
            }
            adj.rotate_right(1);
        }
        adj.swap(0, 2);
        self.flip = true;
        for i in 0..4 {
            if ((id == 0 && adj[side].is_empty()) || (!adj[side].is_empty() && adj[side][0].id == id))
                && ((id2 == 0 && adj[side2].is_empty())
                    || (!adj[side2].is_empty() && adj[side2][0].id == id2))
            {
                self.rotation = i;
                return;
            }
            adj.rotate_right(1);
        }
    }
}

#[aoc("27803643063307", "1644")]
pub fn solve(timer: &mut Timer, input: &str) -> Result<AocResult> {
    let line = Regex::new(r"(\r?\n){2}")?;
    let input: BTreeMap<usize, _> = line
        .split(input)
        .map(|x| {
            let parts = x.split_once('\n').unwrap();
            let id = parts.0[5..=8].parse().unwrap();
            let data = parts.1.replace('.', "0").replace('#', "1");
            let data_map: Vec<_> = data.lines().collect();
            let chars = data_map
                .iter()
                .skip(1)
                .take(8)
                .map(|x| x.chars().skip(1).take(8).collect())
                .collect();

            let top = usize::from_str_radix(data_map[0], 2).unwrap();
            let right = usize::from_str_radix(
                &data_map
                    .iter()
                    .map(|x| x.chars().rev().next().unwrap())
                    .collect::<String>(),
                2,
            )
            .unwrap();
            let bottom = usize::from_str_radix(
                &data_map[data_map.len() - 1]
                    .chars()
                    .rev()
                    .collect::<String>(),
                2,
            )
            .unwrap();
            let left = usize::from_str_radix(
                &data_map
                    .iter()
                    .rev()
                    .map(|x| x.chars().next().unwrap())
                    .collect::<String>(),
                2,
            )
            .unwrap();

            let top_flip =
                usize::from_str_radix(&data_map[0].chars().rev().collect::<String>(), 2).unwrap();
            let right_flip = usize::from_str_radix(
                &data_map
                    .iter()
                    .rev()
                    .map(|x| x.chars().rev().next().unwrap())
                    .collect::<String>(),
                2,
            )
            .unwrap();
            let bottom_flip =
                usize::from_str_radix(&data_map[data_map.len() - 1].chars().collect::<String>(), 2)
                    .unwrap();
            let left_flip = usize::from_str_radix(
                &data_map
                    .iter()
                    .map(|x| x.chars().next().unwrap())
                    .collect::<String>(),
                2,
            )
            .unwrap();

            (
                id,
                RefCell::new(Tile {
                    data: chars,
                    sides: [top, right, bottom, left],
                    flip_sides: [top_flip, right_flip, bottom_flip, left_flip],
                    adjacency: [vec![], vec![], vec![], vec![]],
                    rotation: 0,
                    flip: false,
                }),
            )
        })
        .collect();

    timer.lap("Parse");

    for (&id, image) in &input {
        let mut adjacency = [vec![], vec![], vec![], vec![]];
        for (side_id, &side) in image.borrow().sides.iter().enumerate() {
            for (&test_id, test_image) in &input {
                if test_id == id {
                    continue;
                }
                for (test_side_id, &test_side) in test_image.borrow().sides.iter().enumerate() {
                    if side == test_side {
                        adjacency[side_id].push(Adjacency {
                            id: test_id,
                            side: test_side_id + 2,
                            flip: false,
                        })
                    }
                    if image.borrow().flip_sides[side_id] == test_side {
                        adjacency[side_id].push(Adjacency {
                            id: test_id,
                            side: test_side_id,
                            flip: true,
                        })
                    }
                }
            }
        }
        image.borrow_mut().adjacency = adjacency;
    }

    // it apears that adjacency lists contain either 0 or 1 adjacency, which
    // makes things significantly easier assuming this holds

    let adjacency_sums: Vec<_> = input
        .iter()
        .map(|(&id, x)| {
            (
                id,
                x.borrow().adjacency.iter().map(|x| x.len()).sum::<usize>(),
            )
        })
        .collect();

    let part1 = adjacency_sums
        .iter()
        .fold(1, |acc, (id, val)| if *val == 2 { acc * id } else { acc });

    timer.lap("Part 1");

    let image_size = (input.len() as f32).sqrt() as usize;
    let mut image = vec![vec![0usize; image_size]; image_size];

    // first cell
    image[0][0] = adjacency_sums
        .iter()
        .find(|(_, v)| *v == 2)
        .unwrap()
        .0;
    input[&image[0][0]].borrow_mut().set_transform(0, 0, 3, 0);

    // first row
    for i in 1..image_size {
        let left = image[0][i - 1];
        let new_id = input[&left].borrow().get_side(1).unwrap();
        image[0][i] = new_id;
        input[&new_id].borrow_mut().set_transform(0, 0, 3, left);
    }

    // first column
    for i in 1..image_size {
        let above = image[i - 1][0];
        let new_id = input[&above].borrow().get_side(2).unwrap();
        image[i][0] = new_id;
        input[&new_id].borrow_mut().set_transform(0, above, 3, 0);
    }

    // other cells
    for col in 1..image_size {
        for row in 1..image_size {
            let above = image[row - 1][col];
            let left = image[row][col - 1];
            let new_id = input[&above].borrow().get_side(2).unwrap();
            image[row][col] = new_id;
            input[&new_id].borrow_mut().set_transform(0, above, 3, left);
        }
    }

    for tile in input.values() {
        let mut tile = tile.borrow_mut();
        if tile.flip {
            tile.data.reverse();
        }

        for _ in 0..tile.rotation {
            let mut new = vec![vec![]; 8];
            for (col, n) in new.iter_mut().enumerate() {
                for row in (0..8).rev() {
                    n.push(tile.data[row][col]);
                }
            }
            tile.data = new;
        }
    }

    let mut i = vec![];
    for row in &image {
        for tile_row in 0..8 {
            let mut row_str: Vec<char> = vec![];
            for tile in row {
                row_str.extend(input[&tile].borrow().data[tile_row].iter());
            }
            i.push(row_str);
        }
    }

    fn check_monster(
        i: &[Vec<char>],
        quick_exit: bool,
        found_set: &mut HashSet<(usize, usize)>,
    ) -> i32 {
        let mut count = 0;
        for r in 0..(i.len() - 2) {
            for c in 0..(i.len() - 19) {
                //                   #
                // #    ##    ##    ###
                //  #  #  #  #  #  #
                let monster = [
                    (r, c + 18),
                    (r + 1, c),
                    (r + 1, c + 5),
                    (r + 1, c + 6),
                    (r + 1, c + 11),
                    (r + 1, c + 12),
                    (r + 1, c + 17),
                    (r + 1, c + 18),
                    (r + 1, c + 19),
                    (r + 2, c + 1),
                    (r + 2, c + 4),
                    (r + 2, c + 7),
                    (r + 2, c + 10),
                    (r + 2, c + 13),
                    (r + 2, c + 16),
                ];
                let is_monster = monster.iter().all(|&(r, c)| i[r][c] == '1');
                if is_monster && quick_exit {
                    return 1;
                }
                if is_monster {
                    count += 1;
                    for coord in &monster {
                        found_set.insert(*coord);
                    }
                }
            }
        }

        count
    }

    'out: {
        for _ in 0..4 {
            if check_monster(&i, true, &mut HashSet::new()) > 0 {
                break 'out;
            }

            let mut new = vec![vec![]; i.len()];
            for (col, n) in new.iter_mut().enumerate() {
                for row in (0..i.len()).rev() {
                    n.push(i[row][col]);
                }
            }
            i = new;
        }
        i.reverse();
        for _ in 0..4 {
            if check_monster(&i, true, &mut HashSet::new()) > 0 {
                break 'out;
            }

            let mut new = vec![vec![]; i.len()];
            for (col, n) in new.iter_mut().enumerate() {
                for row in (0..i.len()).rev() {
                    n.push(i[row][col]);
                }
            }
            i = new;
        }

        break 'out;
    }

    let ones = i.iter().fold(0, |acc, row| {
        acc + row
            .iter()
            .fold(0, |acc, tile| if *tile == '1' { acc + 1 } else { acc })
    });

    let mut found_set = HashSet::new();
    check_monster(&i, false, &mut found_set);
    let part2 = ones - found_set.len();

    timer.lap("Part 2");

    Ok(AocResult::new(part1, part2))
}
