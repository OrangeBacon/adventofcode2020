use anyhow::Result;
use hashbrown::HashSet;
use libaoc::{aoc, AocResult, Timer};
use regex::Regex;
use std::collections::VecDeque;
use std::iter::FromIterator;

fn recurse_combat(players: &[VecDeque<usize>], recurse: bool) -> (usize, VecDeque<usize>) {
    let mut players = players.to_vec();

    let mut decks: HashSet<VecDeque<usize>> = HashSet::new();
    loop {
        let play1 = players[0].pop_front().unwrap();
        let play2 = players[1].pop_front().unwrap();

        let winner = if recurse && players[0].len() >= play1 && players[1].len() >= play2 {
            let new_cards = &[
                players[0].range(0..play1).copied().collect(),
                players[1].range(0..play2).copied().collect(),
            ];
            recurse_combat(new_cards, recurse).0 == 0
        } else {
            play1 > play2
        };

        if winner {
            players[0].push_back(play1);
            players[0].push_back(play2);
        } else {
            players[1].push_back(play2);
            players[1].push_back(play1);
        }

        if players[0].len() == 0 {
            break (1, players[1].clone());
        } else if players[1].len() == 0 {
            break (0, players[0].clone());
        }

        if recurse {
            if decks.contains(&players[0]) {
                break (0, players[0].clone());
            } else {
                decks.insert(players[0].clone());
            }
        }
    }
}

#[aoc("32401", "31436")]
pub fn solve(timer: &mut Timer, input: &str) -> Result<AocResult> {
    let line = Regex::new(r"(\r?\n){2}")?;

    let input: Vec<VecDeque<usize>> = line
        .split(input)
        .map(|x| x.lines().skip(1).map(|x| x.parse().unwrap()))
        .map(VecDeque::from_iter)
        .collect();
    timer.lap("Parse");

    let part1 = recurse_combat(&input, false)
        .1
        .iter()
        .rev()
        .enumerate()
        .fold(0, |a, (i, s)| a + (i + 1) * s);

    timer.lap("Part 1");

    let part2 = recurse_combat(&input, true)
        .1
        .iter()
        .rev()
        .enumerate()
        .fold(0, |a, (i, s)| a + (i + 1) * s);

    timer.lap("Part 2");

    Ok(AocResult::new(part1, part2))
}
