use anyhow::Result;
use libaoc::{aoc, AocResult, Timer};

fn run(numbers_map: &mut [(i32, usize, usize)], numbers: &[usize], max: usize) -> usize {
    let mut spoken = 0;
    let len = numbers.len();
    for i in 0..max {
        let num = numbers[i % len];
        if i < numbers.len() {
            numbers_map[num] = (1, i, 0);
            spoken = num;
        } else if numbers_map[spoken] != (0, 0, 0) {
            let last = &mut numbers_map[spoken];
            if last.0 == 1 {
                spoken = 0;
                let num = &mut numbers_map[0];
                num.0 += 1;
                num.2 = num.1;
                num.1 = i;
            } else {
                spoken = last.1 - last.2;
                let mut num = numbers_map[spoken];
                if num == (0, 0, 0) {
                    num = (1, i, 0);
                }
                num.0 += 1;
                num.2 = num.1;
                num.1 = i;
                numbers_map[spoken] = num;
            }
        } else {
            numbers_map[spoken] = (1, i, 0);
            spoken = 0;
        }
    }
    spoken
}

#[aoc("1015", "201", "2020, 30000000, 19, 0, 5, 1, 10, 13")]
pub fn solve(timer: &mut Timer, input: &str) -> Result<AocResult> {
    let numbers: Vec<_> = input
        .split(',')
        .map(|x| x.trim().parse())
        .collect::<Result<_, _>>()?;
    let mut map1 = vec![(0, 0, 0); numbers[0]];
    let mut map2 = vec![(0, 0, 0); numbers[1]];
    let numbers = &numbers[2..];
    timer.lap("Parse");

    let part1 = run(&mut map1, &numbers, 2020);
    timer.lap("Part 1");

    let part2 = run(&mut map2, &numbers, 30000000);
    timer.lap("Part 2");

    Ok(AocResult::new(part1, part2))
}
