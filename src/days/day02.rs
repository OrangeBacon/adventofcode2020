use libaoc::AocResult;
use anyhow::Result;
use regex::Regex;
use std::time::Instant;

pub fn day02(input: String) -> Result<AocResult> {
    let parse = Instant::now();

    let lines = input.lines();
    let extract = Regex::new(r"(\d+)-(\d+) (.): (.+)")?;

    let parse = parse.elapsed().as_secs_f64();

    let time = Instant::now();

    let mut part1 = 0;
    let mut part2 = 0;
    for line in lines {
        let matches = extract.captures(line).unwrap();
        let min = matches.get(1).unwrap().as_str().parse::<usize>()?;
        let max = matches.get(2).unwrap().as_str().parse::<usize>()?;
        let c = matches.get(3).unwrap().as_str().chars().next().unwrap();
        let password = matches.get(4).unwrap().as_str();

        let c_count = password.matches(c).count();
        if c_count >= min && c_count <= max {
            part1 += 1;
        }

        let min_c = password.chars().nth(min - 1).unwrap();
        let max_c = password.chars().nth(max - 1).unwrap();
        if (min_c == c && max_c != c) || (min_c != c && max_c == c) {
            part2 += 1;
        }
    }
    let time = time.elapsed().as_secs_f64();

    Ok(AocResult::new(part1, part2, parse, time, 0.0))
}

#[cfg(test)]
mod test {
    use crate::days::*;
    use anyhow::Result;

    #[test]
    fn day02a() -> Result<()> {
        let res = day02::day02(DEFAULT_DATA[1].to_string())?;
        assert_eq!(res.part1, "483");
        Ok(())
    }

    #[test]
    fn day02b() -> Result<()> {
        let res = day02::day02(DEFAULT_DATA[1].to_string())?;
        assert_eq!(res.part2, "482");
        Ok(())
    }
}
