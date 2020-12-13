use anyhow::Result;
use libaoc::{aoc, AocResult, Timer};

#[aoc("3215", "1001569619313439")]
pub fn solve(timer: &mut Timer, input: String) -> Result<AocResult> {
    let lines: Vec<_> = input.lines().collect();
    let secs: usize = lines[0].parse().unwrap();
    let buses: Vec<usize> = lines[1]
        .split(',')
        .map(|x| x.parse())
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap())
        .collect();
    timer.lap("Parse part 1");
    let mut all_buses: Vec<Option<(usize, usize)>> = lines[1]
        .split(',')
        .map(|x| x.parse())
        .enumerate()
        .map(|x| {
            if x.1.is_ok() {
                Some((x.0, x.1.unwrap()))
            } else {
                None
            }
        })
        .collect();
    all_buses.sort();
    all_buses.reverse();
    timer.lap("Parse part 2");

    let mut current = secs;
    loop {
        if buses.iter().any(|x| current % x == 0) {
            break;
        }
        current += 1;
    }
    let part1 = (current - secs) * buses.iter().find(|&&x| current % x == 0).unwrap();
    timer.lap("Part 1");

    let mut start = 0;
    let mut increment = 1;
    for bus in &all_buses {
        if let Some(bus) = bus {
            let index = bus.0;
            let num = bus.1;
            loop {
                start += increment;
                if (start + index) % num == 0 {
                    break;
                }
            }
            let mut new_increment = start;
            loop {
                new_increment += increment;
                if (new_increment + index) % num == 0 {
                    break;
                }
            }
            increment = new_increment - start;
        }
    }
    let part2 = start;
    timer.lap("Part 2");

    Ok(AocResult::new(part1, part2))
}
