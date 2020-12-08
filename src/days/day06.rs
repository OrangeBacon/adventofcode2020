use crate::{time, AocResult};
use anyhow::Result;
use hashbrown::HashSet;
use std::iter::FromIterator;
use std::time::Instant;

pub fn day06(input: String) -> Result<AocResult> {
    let parse = Instant::now();
    let chars1 = input
        .split("\n\n")
        .map(|x| x.chars().filter(|&x| x != '\n'));

    let chars2 = input.split("\n\n").map(|x| {
        x.lines()
            .map(|x| x.chars().collect())
            .collect::<Vec<Vec<_>>>()
    });
    let parse = parse.elapsed().as_secs_f64();

    let (part1, t1) = time(|| {
        chars1.fold(0, |acc, group| {
            HashSet::<char>::from_iter(group).len() + acc
        })
    });

    let (part2, t2) = time(|| {
        chars2.fold(0, |acc, group| {
            group
                .iter()
                .map(|x| HashSet::from_iter(x.iter().map(|x| *x)))
                .fold(
                    HashSet::from_iter(group[0].iter().map(|x| *x)),
                    |a, person: HashSet<char>| &a & &person,
                )
                .len()
                + acc
        })
    });

    Ok(AocResult::new(part1, part2, parse, t1, t2))
}
