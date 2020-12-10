use anyhow::Result;
use libaoc::{aoc, AocResult};
use std::collections::BTreeMap;
use std::time::Instant;

#[aoc("1885", "2024782584832")]
pub fn solve(input: String) -> Result<AocResult> {
    let parse = Instant::now();
    let mut nums: Vec<_> = input.lines().map(|x|x.parse::<i64>().unwrap()).collect();
    nums.sort();
    let parse = parse.elapsed().as_secs_f64();

    let t1 = Instant::now();
    let mut differences: BTreeMap<i64,i64> = BTreeMap::new();
    let mut outlet = 0;
    for &adaptor in &nums {
        let difference = adaptor - outlet;
        outlet = adaptor;

        differences.insert(difference, 1 + differences.get(&difference).unwrap_or(&0));
    }
    let part1 = differences[&1] * (differences[&3] + 1);
    let t1 = t1.elapsed().as_secs_f64();

    let t2 = Instant::now();
    fn get_options(nums: &[i64]) -> Vec<Vec<(usize,i64)>> {
        nums.iter().enumerate().map(|(i,x)|{
            let mut options = vec![];
            for (j, a) in nums[(i+1)..].iter().enumerate() {
                if a - x < 4 {options.push((j+i+1,*a))} else {break;}
            }
            options
        }).filter(|x|x.len()>0).collect()
    }

    fn recurse(map: &mut BTreeMap<usize,i64>, options: &Vec<Vec<(usize,i64)>>, idx: usize) -> i64 {
        let mut count = 0;
        for &(new_idx,_) in &options[idx] {
            if new_idx == options.len() {
                count += 1;
            } else {
                count += if let Some(&a) = map.get(&new_idx) {
                    a
                } else {
                    recurse(map, options, new_idx)
                }
            }
        }
        map.insert(idx, count);
        count
    }

    println!("{:?}", nums.len());

    let max = *nums.iter().max().unwrap()+3;
    nums.push(max);
    let mut new_nums = vec![0];
    new_nums.append(&mut nums);

    let mut map = BTreeMap::new();
    let options = get_options(&new_nums);
    let part2 = recurse(&mut map, &options, 0);

    let t2 = t2.elapsed().as_secs_f64();

    Ok(AocResult::new(part1, part2, parse, t1, t2))
}
