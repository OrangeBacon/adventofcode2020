use anyhow::Result;
use hashbrown::HashMap;
use libaoc::{aoc, AocResult, Timer};
use regex::Regex;

#[derive(Debug)]
enum Command {
    Mask(u64, u64, Vec<usize>),
    Mem(u64, u64),
}
use Command::*;

#[aoc("11501064782628", "5142195937660")]
pub fn solve(timer: &mut Timer, input: &str) -> Result<AocResult> {
    let mem_reg = Regex::new(r"mem\[(\d+)\] = (\d+)")?;
    let mask_reg = Regex::new(r"mask = ([X01]+)")?;

    let lines: Vec<_> = input
        .lines()
        .map(|x| {
            if let Some(cap) = mem_reg.captures(x) {
                Mem(cap[1].parse().unwrap(), cap[2].parse().unwrap())
            } else {
                let cap = mask_reg.captures(x).unwrap();
                let mut zero_mask = 0;
                let mut one_mask = 0;
                let mut x_mask = vec![];
                for (i, c) in cap[1].chars().rev().enumerate() {
                    match c {
                        '0' => zero_mask |= 1 << i,
                        '1' => one_mask |= 1 << i,
                        'X' => x_mask.push(i),
                        _ => (),
                    }
                }
                Mask(!zero_mask, one_mask, x_mask)
            }
        })
        .collect();

    timer.lap("Parse");

    let mut zero_mask = 0;
    let mut one_mask = 0;
    let mut mem = HashMap::new();
    for line in &lines {
        match line {
            Mask(a, b, _) => {
                zero_mask = *a;
                one_mask = *b;
            }
            Mem(a, b) => {
                mem.insert(a, (b | one_mask) & zero_mask);
            }
        }
    }

    let part1: u64 = mem.values().sum();
    timer.lap("Part 1");

    let mut one_mask = 0;
    let mut x_mask = &vec![];
    let mut mem = HashMap::new();
    for line in &lines {
        match line {
            Mask(_, b, c) => {
                one_mask = *b;
                x_mask = c;
            }
            Mem(addr, value) => {
                let addr = addr | one_mask;
                for curr_change in 0..(2usize.pow(x_mask.len() as u32)) {
                    let mut addr = addr;
                    for (i, addr_index) in x_mask.iter().enumerate() {
                        if (curr_change & (1 << i)) != 0 {
                            addr &= !(1 << addr_index);
                        } else {
                            addr |= 1 << addr_index;
                        }
                    }
                    mem.insert(addr, *value);
                }
            }
        }
    }

    let part2: u64 = mem.values().sum();
    timer.lap("Part 2");

    Ok(AocResult::new(part1, part2))
}
