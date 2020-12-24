use anyhow::Result;
use libaoc::{aoc, AocResult, Timer};
use std::fmt;

struct LinkedList {
    indicies: Vec<usize>,
}

impl LinkedList {
    fn from(data: &[usize]) -> Self {
        let mut indicies = vec![(0, 0); data.len()];
        for (i, num) in indicies.iter_mut().enumerate() {
            *num = (data[i], *data.get(i + 1).unwrap_or(&data[0]) - 1)
        }
        indicies.sort_unstable_by(|&(a, _), &(b, _)| a.cmp(&b));
        let indicies: Vec<_> = indicies.iter().map(|&(_, i)| i).collect();
        LinkedList { indicies }
    }

    fn remove_after(&mut self, start: usize, count: usize) -> usize {
        let ret = self.indicies[start];

        let mut end = start;
        for _ in 0..=count {
            end = self.indicies[end];
        }

        self.indicies[start] = end;

        ret
    }

    fn insert_after(&mut self, index: usize, data_ptr: usize, data_len: usize) {
        let mut end = data_ptr;
        for _ in 0..(data_len - 1) {
            end = self.indicies[end];
        }

        self.indicies[end] = self.indicies[index];

        self.indicies[index] = data_ptr;
    }

    fn to_vec(&self) -> Vec<usize> {
        let mut result = Vec::with_capacity(self.indicies.len());
        result.push(1);

        let mut index = 0;
        for _ in 0..(self.indicies.len() - 1) {
            let data = self.indicies[index];
            result.push(data + 1);
            index = data
        }

        result
    }
}

impl fmt::Debug for LinkedList {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self.to_vec())
    }
}

fn game(cups: &[usize], move_count: usize) -> LinkedList {
    let max = cups.len();
    let mut current_cup = cups[0] - 1;
    let mut cups = LinkedList::from(cups);

    for _ in 0..move_count {
        let c1 = cups.indicies[current_cup];
        let c2 = cups.indicies[c1];
        let c3 = cups.indicies[c2];
        let moved_cups = [c1 + 1, c2 + 1, c3 + 1];

        let removed = cups.remove_after(current_cup, 3);

        let mut destination = current_cup;
        loop {
            if destination == 0 {
                destination = max;
            }
            if moved_cups.contains(&destination) {
                destination -= 1
            } else {
                break;
            }
        }

        cups.insert_after(destination - 1, removed, 3);

        current_cup = cups.indicies[current_cup];
    }

    cups
}

#[aoc("82635947", "157047826689", "685974213")]
pub fn solve(timer: &mut Timer, input: &str) -> Result<AocResult> {
    let mut cups: Vec<_> = input
        .chars()
        .map(|x| x.to_digit(10).unwrap() as usize)
        .collect();
    timer.lap("Parse");

    let mut part1 = game(&cups, 100).to_vec();
    while part1[0] != 1 {
        part1.rotate_right(1);
    }
    let part1 = part1[1..]
        .iter()
        .fold(String::new(), |s, &c| s + &c.to_string());
    timer.lap("Part 1");

    cups.extend(10..=1_000_000);
    let part2 = game(&cups, 10_000_000);
    let part2 = (part2.indicies[0] + 1) * (part2.indicies[part2.indicies[0]] + 1);

    timer.lap("Part 2");

    Ok(AocResult::new(part1, part2))
}
