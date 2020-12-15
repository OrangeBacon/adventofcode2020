use anyhow::Result;
use libaoc::{aoc, AocResult, Timer};
use regex::Regex;

#[aoc("228", "175")]
pub fn solve(timer: &mut Timer, input: &str) -> Result<AocResult> {
    let line = Regex::new(r"(\r?\n){2}")?;

    let datas: Vec<Vec<Vec<_>>> = line
        .split(&input)
        .map(|x| {
            x.split_whitespace()
                .map(|y| y.split(':').collect())
                .collect()
        })
        .collect();
    timer.lap("Parse");

    let birth = Regex::new(r"19[2-9][0-9]|200[0-2]")?;
    let issue = Regex::new(r"201[0-9]|2020")?;
    let expire = Regex::new(r"202[0-9]|2030")?;
    let height = Regex::new(r"^(?:1[5-8][0-9]cm|19[0-3]cm|59in|6[0-9]in|7[0-6]in)$")?;
    let hair = Regex::new(r"#[0-9a-f]{6}")?;
    let eye = Regex::new(r"amb|blu|brn|gry|grn|hzl|oth")?;
    let pid = Regex::new(r"^[0-9]{9}$")?;
    timer.lap("Regex Compilation");

    let res = datas
        .iter()
        .filter(|x| x.len() == 8 || (x.len() == 7 && x.iter().find(|y| y[0] == "cid").is_none()));
    let part1 = res.clone().count();
    timer.lap("Part 1");

    let mut part2 = 0;
    for pass in res {
        let mut passed = true;
        for sec in pass {
            if !match sec[0] {
                "byr" => birth.is_match(sec[1]),
                "iyr" => issue.is_match(sec[1]),
                "eyr" => expire.is_match(sec[1]),
                "hgt" => height.is_match(sec[1]),
                "hcl" => hair.is_match(sec[1]),
                "ecl" => eye.is_match(sec[1]),
                "pid" => pid.is_match(sec[1]),
                "cid" => true,
                _ => panic!(),
            } {
                passed = false;
                break;
            }
        }
        if passed {
            part2 += 1;
        }
    }
    timer.lap("Part 2");

    Ok(AocResult::new(part1, part2))
}
