use hashbrown::HashSet;
use std::iter::FromIterator;
use anyhow::Result;
use crate::AocResult;

pub fn day06(input: String) -> Result<AocResult> {
    let chars = input
        .split("\n\n")
        .map(|x| x.chars().filter(|&x| x != '\n'));

    let part1 = chars.fold(0, |acc, group| {
        HashSet::<char>::from_iter(group).len() + acc
    });

    let chars = input.split("\n\n").map(|x| {
        x.lines()
            .map(|x| x.chars().collect())
            .collect::<Vec<Vec<_>>>()
    });

    let part2 = chars.fold(0, |acc, group| {
        group
            .iter()
            .map(|x| HashSet::from_iter(x.iter().map(|x| *x)))
            .fold(
                HashSet::from_iter(group[0].iter().map(|x| *x)),
                |a, person: HashSet<char>| &a & &person,
            )
            .len()
            + acc
    });

    Ok(AocResult::new(part1, part2))
}
