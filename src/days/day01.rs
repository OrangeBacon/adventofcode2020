use anyhow::Result;
use crate::AocResult;

pub fn day01(input: String) -> Result<AocResult> {
    let nums = input
        .split_whitespace()
        .map(|x| x.parse::<i32>())
        .collect::<Result<Vec<_>,_>>()?;

    let mut part1 = 0;
    'out: for (i, num) in nums.iter().enumerate() {
        for second in nums[(i + 1)..].iter() {
            if num + second == 2020 {
                part1 = num * second;
                break 'out;
            }
        }
    }

    let mut part2 = 0;
    'out2: for (i, num) in nums.iter().enumerate() {
        for (j, second) in nums[(i + 1)..].iter().enumerate() {
            for third in nums[(j + 1)..].iter() {
                if num + second + third == 2020 {
                    part2 = num * second * third;
                    break 'out2;
                }
            }
        }
    }

    Ok(AocResult::new(part1, part2))
}
