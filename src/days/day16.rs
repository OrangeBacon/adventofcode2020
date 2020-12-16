use anyhow::Result;
use libaoc::{aoc, AocResult, Timer};
use regex::Regex;
use std::ops::RangeInclusive;
use bitvec::prelude::*;

#[derive(Debug, Clone)]
struct Constraint {
    name: String,
    range1: RangeInclusive<i64>,
    range2: RangeInclusive<i64>,
}

#[aoc("27802", "279139880759")]
pub fn solve(timer: &mut Timer, input: &str) -> Result<AocResult> {
    let line = Regex::new(r"(\r?\n){2}")?;
    let constraint = Regex::new(r"([a-z ]+): (\d+)-(\d+) or (\d+)-(\d+)")?;
    timer.lap("Compile regex");

    let parts: Vec<_> = line.split(input).collect();
    let constraints: Vec<_> = parts[0]
        .split('\n')
        .map(|x| constraint.captures(x).unwrap())
        .map(|x| Constraint {
            name: x[1].to_string(),
            range1: (x[2].parse().unwrap())..=(x[3].parse().unwrap()),
            range2: (x[4].parse().unwrap())..=(x[5].parse().unwrap()),
        })
        .collect();
    timer.lap("Parse constraints");

    let my_ticket: Vec<i64> = parts[1]
        .trim()
        .split('\n')
        .nth(1)
        .unwrap()
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    timer.lap("Parse my ticket");

    let tickets: Vec<Vec<i64>> = parts[2]
        .trim()
        .split('\n')
        .skip(1)
        .map(|x| x.split(',').map(|x| x.trim().parse().unwrap()).collect())
        .collect();
    timer.lap("Parse nearby tickets");

    let mut values = bitarr![0; 1000];
    for constraint in &constraints {
        for i in constraint.range1.clone() {
            values.set(i as usize, true);
        }
        for i in constraint.range2.clone() {
            values.set(i as usize, true);
        }
    }

    let mut total = 0;
    let mut invalid = vec![];
    for (i, ticket) in tickets.iter().enumerate() {
        for num in ticket {
            if !values[*num as usize] {
                total += num;
                invalid.push(i);
                break;
            }
        }
    }
    let part1 = total;

    timer.lap("Part 1");

    let tickets: Vec<_> = tickets
        .iter()
        .enumerate()
        .filter(|(i, _)| !invalid.contains(i))
        .map(|(_, x)| x)
        .collect();
    timer.lap("filter");

    let mut fields = vec![Vec::with_capacity(tickets[0].len()); tickets[0].len()];
    for (i, constraint) in constraints.iter().enumerate() {
        'loop1: for field in 0..tickets[0].len() {
            for ticket in &tickets {
                if !(constraint.range1.contains(&ticket[field])
                    || constraint.range2.contains(&ticket[field]))
                {
                    continue 'loop1;
                }
            }
            fields[field].push(i as i64);
        }
    }

    timer.lap("graph");

    let len = fields.len();
    let mut removed = bitarr![0; 20];
    for _ in 0..len {
        let remove = fields
            .iter()
            .find(|x| x.len() == 1 && !removed[x[0] as usize])
            .unwrap()[0];
        for vec in fields.iter_mut() {
            if vec.len() != 1 {
                vec.retain(|&x| x != remove);
            }
        }

        removed.set(remove as usize, true);
    }
    timer.lap("reduce");

    let part2 = fields
        .iter()
        .enumerate()
        .fold(1, |acc, (index, constraint_no)| {
            if constraint_no[0] < 6 {
                acc * my_ticket[index]
            } else {
                acc
            }
        });

    timer.lap("Part 2");

    Ok(AocResult::new(part1, part2))
}
