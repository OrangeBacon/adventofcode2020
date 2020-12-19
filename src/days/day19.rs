use anyhow::Result;
use libaoc::{aoc, AocResult, Timer};
use regex::Regex;
use pcre2::bytes::Regex as RegexPcre2;
use hashbrown::HashMap;
use std::fmt::{self, Write};

#[derive(Clone, Copy, Debug)]
enum RuleType {
    One(usize),
    Two(usize, usize),
    OneOne(usize, usize),
    TwoTwo(usize, usize, usize, usize),
    Letter(char),
    Rule8(usize),
    Rule11(usize, usize),
}

impl RuleType {
    fn to_regex(self, out: &mut String, rules: &HashMap<usize, RuleType>) -> Result<(), fmt::Error> {
        use RuleType::*;

        match self {
            Letter(a) => write!(out, "{}", a),
            One(a) => rules[&a].to_regex(out, rules),
            Two(a, b) => {
                rules[&a].to_regex(out, rules)?;
                rules[&b].to_regex(out, rules)
            }
            OneOne(a,b) => {
                write!(out, "((")?;
                rules[&a].to_regex(out, rules)?;
                write!(out, ")|(")?;
                rules[&b].to_regex(out, rules)?;
                write!(out, "))")
            }
            TwoTwo(a,b,c,d) => {
                write!(out, "((")?;
                rules[&a].to_regex(out, rules)?;
                rules[&b].to_regex(out, rules)?;
                write!(out, ")|(")?;
                rules[&c].to_regex(out, rules)?;
                rules[&d].to_regex(out, rules)?;
                write!(out, "))")
            }
            Rule8(a) => {
                write!(out, "(")?;
                rules[&a].to_regex(out, rules)?;
                write!(out, ")+")
            }
            Rule11(a,b) => {
                write!(out, "(?<rule11>(")?;
                rules[&a].to_regex(out, rules)?;
                rules[&b].to_regex(out, rules)?;
                write!(out, ")|(")?;
                rules[&a].to_regex(out, rules)?;
                write!(out, "(?&rule11)")?;
                rules[&b].to_regex(out, rules)?;
                write!(out, "))")
            }
        }
    }

    fn from_str(x: &str) -> (usize, Self) {
        use RuleType::*;

        let parts = x.split_once(':').unwrap();
        let left = parts.0.parse().unwrap();
        let right = parts.1.split('|');
        let right: Vec<Vec<(&str, Result<usize, _>)>> = right
            .map(|x| x.trim().split(' ').map(|x| (x, x.parse())).collect())
            .collect();

        let kind = match right.len() {
            1 => match right[0].len() {
                2 => Two(*(right[0][0].1.as_ref().unwrap()), *(right[0][1].1.as_ref().unwrap())),
                1 => {
                    if let Ok(num) = right[0][0].1 {
                        One(num)
                    } else {
                        Letter(right[0][0].0.chars().nth(1).unwrap())
                    }
                }
                _ => panic!(),
            },
            2 => match right[0].len() {
                1 => OneOne(
                    *(right[0][0].1.as_ref().unwrap()),
                    *(right[1][0].1.as_ref().unwrap()),
                ),
                2 => TwoTwo(
                        *(right[0][0].1.as_ref().unwrap()),
                        *(right[0][1].1.as_ref().unwrap()),
                        *(right[1][0].1.as_ref().unwrap()),
                        *(right[1][1].1.as_ref().unwrap()),
                    ),
                _ => panic!(),
            },
            _ => panic!(),
        };

        (left, kind)
    }
}

#[aoc("250", "359")]
pub fn solve(timer: &mut Timer, input: &str) -> Result<AocResult> {
    let line = Regex::new(r"(\r?\n){2}")?;
    let input: Vec<_> = line.split(input).collect();

    let mut rules: HashMap<usize, RuleType> = input[0].lines().map(RuleType::from_str).collect();

    timer.lap("Parse");

    let mut reg = String::new();
    write!(reg, "^(")?;
    rules[&0].to_regex(&mut reg, &rules)?;
    write!(reg, ")$")?;
    let reg = Regex::new(&reg)?;

    let part1 = input[1].lines().fold(0, |acc, line| {
        if reg.is_match(line) {
            acc + 1
        } else {
            acc
        }
    });

    timer.lap("Part 1");

    rules.insert(8, RuleType::Rule8(42));
    rules.insert(11, RuleType::Rule11(42, 31));

    timer.lap("Part 2");

    let mut reg = String::new();
    write!(reg, "^(")?;
    rules[&0].to_regex(&mut reg, &rules)?;
    write!(reg, ")$")?;
    let reg = RegexPcre2::new(&reg)?;

    let part2 = input[1].lines().fold(0, |acc, line| {
        if reg.is_match(&line.bytes().collect::<Vec<u8>>()).unwrap() {
            acc + 1
        } else {
            acc
        }
    });

    Ok(AocResult::new(part1, part2))
}
