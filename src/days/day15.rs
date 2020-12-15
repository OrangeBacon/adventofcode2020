use anyhow::Result;
use hashbrown::HashMap;
use libaoc::{aoc, AocResult, Timer};

fn run(numbers: &[usize], max: usize) -> usize {
    let mut spoken = 0;
    let len = numbers.len();
    let mut numbers_map = HashMap::new();
    for i in 0..max {
        let num = numbers[i % len];
        if i < numbers.len() {
            numbers_map.insert(num, (1, i, 0));
            spoken = num;
        } else if let Some(last) = numbers_map.get_mut(&spoken) {
            if last.0 == 1 {
                spoken = 0;
                let num = numbers_map.get_mut(&0).unwrap();
                num.0 += 1;
                num.2 = num.1;
                num.1 = i;
            } else {
                spoken = last.1 - last.2;
                let temp = (1, i, 0);
                let mut num = *numbers_map.get(&spoken).unwrap_or(&temp);
                num.0 += 1;
                num.2 = num.1;
                num.1 = i;
                numbers_map.insert(spoken, num);
            }
        } else {
            numbers_map.insert(spoken, (1, i, 0));
            spoken = 0;
        }
    }
    spoken
}

#[aoc("1015", "201")]
pub fn solve(timer: &mut Timer, _input: String) -> Result<AocResult> {
    let numbers = vec![19, 0, 5, 1, 10, 13];
    timer.lap("Parse");

    let part1 = run(&numbers, 2020);
    timer.lap("Part 1");

    let part2 = run(&numbers, 30000000);
    timer.lap("Part 2");

    Ok(AocResult::new(part1, part2))
}
