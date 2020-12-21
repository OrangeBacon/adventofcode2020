use anyhow::Result;
use hashbrown::{HashMap, HashSet};
use libaoc::{aoc, AocResult, Timer};
use std::collections::BTreeMap;
use std::iter::FromIterator;

#[derive(Debug)]
struct Line<'a> {
    ingredients: Vec<&'a str>,
    allergens: Vec<&'a str>,
}

#[aoc("2542", "hkflr,ctmcqjf,bfrq,srxphcm,snmxl,zvx,bd,mqvk")]
pub fn solve(timer: &mut Timer, input: &str) -> Result<AocResult> {
    let input = input.lines().map(|x| x.split_once('(').unwrap());

    let input: Vec<_> = input
        .map(|(ingredients, allergens)| Line {
            ingredients: ingredients.trim().split(' ').collect::<Vec<_>>(),
            allergens: allergens.trim()[9..(allergens.len() - 1)]
                .split(", ")
                .collect::<Vec<_>>(),
        })
        .collect();

    timer.lap("Parse");

    let mut ingredients: HashMap<&str, HashSet<&str>> = HashMap::new();
    for line in input.iter() {
        for allergen in &line.allergens {
            if let Some(current) = ingredients.get(allergen) {
                let int = HashSet::from_iter(line.ingredients.iter().copied())
                    .intersection(current)
                    .copied()
                    .collect();
                ingredients.insert(allergen, int);
            } else {
                ingredients.insert(
                    allergen,
                    HashSet::from_iter(line.ingredients.iter().copied()),
                );
            }
        }
    }

    let mut allergens: HashSet<&str> = HashSet::new();
    for allergen in ingredients.values() {
        allergens.extend(allergen);
    }

    let mut all_ingredients: HashMap<&str, usize> = HashMap::new();
    for line in input.iter() {
        for ingredient in &line.ingredients {
            if !allergens.contains(ingredient) {
                let count = all_ingredients.get(ingredient).unwrap_or(&0) + 1;
                all_ingredients.insert(ingredient, count);
            }
        }
    }

    let part1: usize = all_ingredients.values().sum();

    timer.lap("Part 1");

    let mut mappings = BTreeMap::new();
    for _ in 0..ingredients.len() {
        let (name, ones) = ingredients
            .iter()
            .filter(|&(_, x)| x.len() == 1)
            .next()
            .unwrap();
        let value = *ones.iter().next().unwrap();

        mappings.insert(name.clone(), value);

        for (_, list) in ingredients.iter_mut() {
            list.retain(|x| *x != value);
        }
    }

    let part2: Vec<_> = mappings.values().collect();
    let part2 = part2.iter().fold(String::new(), |s, c| {
        if s.len() > 0 {
            s + "," + c
        } else {
            c.to_string()
        }
    });

    timer.lap("Part 2");

    Ok(AocResult::new(part1, part2))
}
