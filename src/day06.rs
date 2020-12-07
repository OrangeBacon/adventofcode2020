use hashbrown::HashSet;
use std::iter::FromIterator;

pub fn day06(input: String) {
    let chars = input.split("\n\n").map(|x|x.chars().filter(|&x|x!='\n'));

    let res = chars.fold(0, |acc, group| HashSet::<char>::from_iter(group).len() + acc);

    println!("{}", res);

    let chars = input.split("\n\n").map(|x|x.lines().map(|x|x.chars().collect()).collect::<Vec<Vec<_>>>());

    let res = chars.fold(0, |acc, group|
        group.iter().map(|x| HashSet::from_iter(x.iter().map(|x|*x)))
        .fold(HashSet::from_iter(group[0].iter().map(|x|*x)), |a, person:HashSet<char>| &a & &person)
        .len() + acc
    );

    println!("{}", res);
}