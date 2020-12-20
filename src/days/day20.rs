use anyhow::Result;
use hashbrown::HashMap;
use libaoc::{aoc, AocResult, Timer};
use regex::Regex;
use std::cell::RefCell;

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
    data: String,
    sides: [usize; 4],
    flip_sides: [usize; 4],
    adjacency: [Vec<Adjacency>; 4],
    rotation: usize,
    flip_horiz: bool,
    flip_vert: bool,
}

impl Tile {
    fn get_side(&self, side: usize) -> usize {
        let mut adj = self.adjacency.to_vec();
        adj.rotate_right(self.rotation);
        if self.flip_horiz {
            adj.swap(1,3);
        }
        if self.flip_vert {
            adj.swap(0,2);
        }
        adj[side][0].id
    }

    /// applies flips and rotation until side side == id
    fn set_transform(&mut self, side: usize, id: usize) {
        let mut pos = self.adjacency.iter().position(|x|x.len() > 0 && x[0].id == id).unwrap();
        let used_adjacency = &self.adjacency[pos][0];

        if used_adjacency.flip {
            if pos == 0 || pos == 3 {
                self.flip_vert = true;
            } else {
                self.flip_horiz = true;
            }

            pos = (pos + 2) % 4;
        }

        let rotate = (side as i32) - (pos as i32);
        if rotate > 0 {
            self.rotation = (rotate as usize) % 4;
        } else {
            self.rotation = ((rotate + 4) as usize) % 4;
        }
    }
}

#[aoc("20899048083289", "")]
pub fn solve(timer: &mut Timer, input: &str) -> Result<AocResult> {
    let line = Regex::new(r"(\r?\n){2}")?;
    let input: HashMap<usize, _> = line
        .split(input)
        .map(|x| {
            let parts = x.split_once('\n').unwrap();
            let id = parts.0[5..=8].parse().unwrap();
            let data = parts.1.replace('.', "0").replace('#', "1");
            let data_map: Vec<_> = data.lines().collect();

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

            let top_flip = usize::from_str_radix(&data_map[0].chars().rev().collect::<String>(), 2).unwrap();
            let right_flip = usize::from_str_radix(
                &data_map
                    .iter().rev()
                    .map(|x| x.chars().rev().next().unwrap())
                    .collect::<String>(),
                2,
            )
            .unwrap();
            let bottom_flip = usize::from_str_radix(
                &data_map[data_map.len() - 1]
                    .chars()
                    .collect::<String>(),
                2,
            )
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
                    data,
                    sides: [top, right, bottom, left],
                    flip_sides: [top_flip, right_flip, bottom_flip, left_flip],
                    adjacency: [vec![], vec![], vec![], vec![]],
                    rotation: 0,
                    flip_horiz: false,
                    flip_vert: false,
                }),
            )
        })
        .collect();

    timer.lap("Parse");

    for (&id, image) in &input {
        let mut adjacency = [vec![], vec![], vec![], vec![]];
        for (side_id, &side) in image.borrow().sides.iter().enumerate() {
            for (&test_id, test_image) in &input {
                if test_id == id {continue;}
                for (test_side_id, &test_side) in test_image.borrow().sides.iter().enumerate() {
                    if side == test_side {
                        adjacency[side_id].push(Adjacency {
                            id: test_id,
                            side: test_side_id,
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

    let adjacency_sums:Vec<_> = input.iter().map(|(&id, x)| {
        (id, x.borrow().adjacency.iter().map(|x|x.len()).sum::<usize>())
    }).collect();

    let part1 = adjacency_sums.iter().fold(1, |acc, (id, val)| {
        if *val == 2 {
            acc * id
        } else {
            acc
        }
    });

    timer.lap("Part 1");

    let image_size = (input.len() as f32).sqrt() as usize;
    let mut image = vec![vec![0usize; image_size]; image_size];

    for &(id,_) in adjacency_sums.iter().filter(|(_,v)|*v==2) {
        let corner = input[&id].borrow();
        let a = &corner.adjacency;

        let is_flip = a.iter().fold(false, |acc, x| {
            acc || (x.len() > 0 && x[0].flip)
        });

        if is_flip {}
        else if a[1].len() == 1 && a[2].len() == 1 {
            image[0][0] = id;
        } else if a[2].len() == 1 && a[3].len() == 1 {
            image[0][image_size-1] = id;
        } else if a[0].len() == 1 && a[1].len() == 1  {
            image[image_size-1][0] = id;
        } else {
            image[image_size-1][image_size-1] = id;
        }
    }

    // turns out for both my input and the test input, this places the single
    // non flipped tile in the top right corner, assume this holds

    // construct first row of image
    for i in (0..(image_size-1)).rev() {
        let right = image[0][i + 1];
        let current_tile_id = input[&right].borrow().get_side(3);
        image[0][i] = current_tile_id;
        input[&right].borrow_mut().set_transform(3, current_tile_id);
    }

    // patch last item in first row, cause not transformed in above loop
    input[&image[0][0]].borrow_mut().set_transform(3, image[0][1]);

    // first column
    /*for i in 1..image_size {
        let above = image[i-1][0];
        println!("{}, {:?}", above, input[&above]);
        let current_tile_id = input[&above].borrow().get_side(2);
        image[i][0] = current_tile_id;
        input[&above].borrow_mut().set_transform(2, current_tile_id);
    }
*/
    println!("{:?}", image);

    timer.lap("Part 2");

    Ok(AocResult::new(part1, ""))
}
