use anyhow::Result;
use libaoc::{aoc, AocResult, Timer};

#[aoc("1885", "2024782584832")]
pub fn solve(timer: &mut Timer, input: &str) -> Result<AocResult> {
    let mut nums: Vec<_> = input.lines().map(|x| x.parse::<i64>().unwrap()).collect();
    nums.sort_unstable();
    timer.lap("Parse");

    let mut outlet = 0;
    let mut ones = 0;
    let mut threes = 1;

    for &adaptor in &nums {
        let difference = adaptor - outlet;
        outlet = adaptor;

        if difference == 1 {
            ones += 1;
        }
        if difference == 3 {
            threes += 1;
        }
    }

    let part1 = ones * threes;
    timer.lap("Part 1");

    fn recurse(map: &mut [i64], options: &[Vec<usize>], idx: usize) -> i64 {
        let count = options[idx].iter().fold(0, |count, &new_idx| {
            count
                + if new_idx == options.len() {
                    1
                } else if map[new_idx] != 0 {
                    map[new_idx]
                } else {
                    recurse(map, options, new_idx)
                }
        });
        map[idx] = count;
        count
    }

    let max = *nums.iter().max().unwrap() + 3;
    nums.push(max);
    let mut new_nums = vec![0];
    new_nums.append(&mut nums);
    let nums = new_nums;

    let options: Vec<_> = nums
        .iter()
        .enumerate()
        .map(|(i, x)| {
            let mut options = vec![];
            for (j, a) in nums[(i + 1)..].iter().enumerate() {
                if a - x < 4 {
                    options.push(j + i + 1)
                } else {
                    break;
                }
            }
            options
        })
        .filter(|x| !x.is_empty())
        .collect();
    timer.lap("Part 2 Collect Options");

    let mut memo = vec![0; options.len()];
    let part2 = recurse(&mut memo, &options, 0);
    timer.lap("Part 2 Recurse");

    Ok(AocResult::new(part1, part2))
}
