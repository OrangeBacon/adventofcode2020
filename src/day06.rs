use std::collections::HashSet;
use std::iter::FromIterator;

pub fn day06(input: String) {
    let chars : Vec<Vec<_>> = input.split("\n\n").map(|x|x.chars().filter(|&x|x!='\n').collect()).collect();

    let res = chars.iter().fold(0, |acc, group| {
        let mut set = HashSet::new();
        for item in group.iter() {set.insert(*item);}
        set.len() + acc
    });

    println!("{}", res);

    let chars : Vec<Vec<Vec<_>>> = input.split("\n\n").map(|x|x.lines().map(|x|x.chars().collect()).collect()).collect();

    let res = chars.iter().fold(0, |acc, group| {
        let res : HashSet<char> = group.iter().fold(HashSet::from_iter(vec!['\n']), |a, person| {
            let mut set = HashSet::new();
            for item in person.iter() {set.insert(*item);}
            let r = if a == HashSet::from_iter(vec!['\n']) {
                set
            } else {
                a.intersection(&set).copied().collect()
            };
            r
        });
        res.len() + acc
    });

    println!("{}", res);
}