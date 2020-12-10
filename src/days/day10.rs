use anyhow::Result;
use libaoc::{aoc, AocResult};
use std::time::Instant;

#[aoc("1885", "2024782584832")]
pub fn solve(input: String) -> Result<AocResult> {
    let parse = Instant::now();
    let mut nums: Vec<_> = input.lines().map(|x|x.parse::<i64>().unwrap()).collect();
    nums.sort();
    let parse = parse.elapsed().as_secs_f64();

    let t1 = Instant::now();
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
    let t1 = t1.elapsed().as_secs_f64();

    let t2 = Instant::now();

    fn recurse(map: &mut [i64], options: &[Vec<usize>], idx: usize) -> i64 {
        let count = options[idx].iter().fold(0, |count, &new_idx| {
            count + if new_idx == options.len() {
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

    let max = *nums.iter().max().unwrap()+3;
    nums.push(max);
    let mut new_nums = vec![0];
    new_nums.append(&mut nums);
    let nums = new_nums;

    let options: Vec<_> = nums.iter().enumerate().map(|(i,x)|{
        let mut options = vec![];
        for (j, a) in nums[(i+1)..].iter().enumerate() {
            if a - x < 4 {options.push(j+i+1)} else {break;}
        }
        options
    }).filter(|x|x.len()>0).collect();

    let mut memo = vec![0; options.len()];
    let part2 = recurse(&mut memo, &options, 0);

    let t2 = t2.elapsed().as_secs_f64();

    Ok(AocResult::new(part1, part2, parse, t1, t2))
}
