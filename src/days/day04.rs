use crate::AocResult;
use anyhow::Result;
use regex::Regex;
use std::time::Instant;

pub fn day04(input: String) -> Result<AocResult> {
    let parse = Instant::now();

    let datas: Vec<Vec<Vec<_>>> = input
        .split("\n\n")
        .map(|x| {
            x.split_whitespace()
                .map(|y| y.split(':').collect())
                .collect()
        })
        .collect();

    let birth = Regex::new(r"19[2-9][0-9]|200[0-2]")?;
    let issue = Regex::new(r"201[0-9]|2020")?;
    let expire = Regex::new(r"202[0-9]|2030")?;
    let height = Regex::new(r"^(?:1[5-8][0-9]cm|19[0-3]cm|59in|6[0-9]in|7[0-6]in)$")?;
    let hair = Regex::new(r"#[0-9a-f]{6}")?;
    let eye = Regex::new(r"amb|blu|brn|gry|grn|hzl|oth")?;
    let pid = Regex::new(r"^[0-9]{9}$")?;
    let cid = Regex::new("")?;
    let parse = parse.elapsed().as_secs_f64();

    let t1 = Instant::now();
    let res = datas
        .iter()
        .filter(|x| x.len() == 8 || (x.len() == 7 && x.iter().find(|y| y[0] == "cid").is_none()));
    let part1 = res.clone().count();
    let t1 = t1.elapsed().as_secs_f64();

    let t2 = Instant::now();
    let mut part2 = 0;
    for pass in res {
        let mut passed = true;
        for sec in pass {
            if !match sec[0] {
                "byr" => &birth,
                "iyr" => &issue,
                "eyr" => &expire,
                "hgt" => &height,
                "hcl" => &hair,
                "ecl" => &eye,
                "pid" => &pid,
                "cid" => &cid,
                _ => panic!(),
            }
            .is_match(sec[1])
            {
                passed = false;
                break;
            }
        }
        if passed {
            part2 += 1;
        }
    }
    let t2 = t2.elapsed().as_secs_f64();

    Ok(AocResult::new(part1, part2, parse, t1, t2))
}
