use crate::AocResult;
use anyhow::Result;
use std::time::Instant;

pub fn day01(input: String) -> Result<AocResult> {
    let parse = Instant::now();

    let nums = input
        .split_whitespace()
        .map(|x| x.parse::<i32>())
        .collect::<Result<Vec<_>, _>>()?;

    let parse = parse.elapsed().as_secs_f64();

    let t1 = Instant::now();
    let mut part1 = 0;
    'out: for (i, num) in nums.iter().enumerate() {
        for second in nums[(i + 1)..].iter() {
            if num + second == 2020 {
                part1 = num * second;
                break 'out;
            }
        }
    }
    let t1 = t1.elapsed().as_secs_f64();

    let t2 = Instant::now();
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
    let t2 = t2.elapsed().as_secs_f64();

    Ok(AocResult::new(part1, part2, parse, t1, t2))
}

#[cfg(test)]
mod test {
    use crate::days::*;
    use anyhow::Result;

    #[test]
    fn day01a() -> Result<()> {
        let res = day01::day01(DEFAULT_DATA[0].to_string())?;
        assert_eq!(res.part1, "964875");
        Ok(())
    }

    #[test]
    fn day01b() -> Result<()> {
        let res = day01::day01(DEFAULT_DATA[0].to_string())?;
        assert_eq!(res.part2, "158661360");
        Ok(())
    }
}