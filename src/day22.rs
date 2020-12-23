use anyhow::{anyhow, Result};
use std::collections::{HashSet, VecDeque};
use std::path::Path;

use crate::reader::split_once;

#[derive(Debug)]
enum Winner {
    Player1(VecDeque<usize>),
    Player2(VecDeque<usize>),
}

impl Winner {
    fn score(&self) -> usize {
        let winner = match self {
            Winner::Player1(c) => c,
            Winner::Player2(c) => c,
        };
        winner
            .iter()
            .rev()
            .enumerate()
            .map(|(i, v)| (i + 1) * v)
            .sum::<usize>()
    }
}

fn combat(mut player_1: VecDeque<usize>, mut player_2: VecDeque<usize>) -> Winner {
    while !player_1.is_empty() && !player_2.is_empty() {
        let a = player_1.pop_front().unwrap();
        let b = player_2.pop_front().unwrap();

        if a > b {
            player_1.push_back(a);
            player_1.push_back(b);
        } else {
            player_2.push_back(b);
            player_2.push_back(a);
        }
    }

    if !player_1.is_empty() {
        Winner::Player1(player_1)
    } else {
        Winner::Player2(player_2)
    }
}

fn recursive_combat(mut player_1: VecDeque<usize>, mut player_2: VecDeque<usize>) -> Winner {
    let mut prev_rounds = HashSet::new();
    while !player_1.is_empty() && !player_2.is_empty() {
        let prev_round = (player_1.clone(), player_2.clone());
        if prev_rounds.contains(&prev_round) {
            return Winner::Player1(player_1);
        }
        prev_rounds.insert(prev_round);

        // Unwrap is safe since we know both player_1 and player_2 are not empty
        let a = player_1.pop_front().unwrap();
        let b = player_2.pop_front().unwrap();

        if player_1.len() < a || player_2.len() < b {
            // Normal game
            if a > b {
                player_1.push_back(a);
                player_1.push_back(b);
            } else {
                player_2.push_back(b);
                player_2.push_back(a);
            }
        } else {
            let sub_game = recursive_combat(
                player_1.iter().copied().take(a).collect(),
                player_2.iter().copied().take(b).collect(),
            );
            match sub_game {
                Winner::Player1(_) => {
                    player_1.push_back(a);
                    player_1.push_back(b);
                }
                Winner::Player2(_) => {
                    player_2.push_back(b);
                    player_2.push_back(a);
                }
            }
        }
    }

    if !player_1.is_empty() {
        Winner::Player1(player_1)
    } else {
        Winner::Player2(player_2)
    }
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let input = std::fs::read_to_string(path)?;
    let (player_1_str, player_2_str) = split_once(&input, "\n\n");

    let player_1 = player_1_str
        .lines()
        .skip(1)
        .map(|l| Ok(l.parse()?))
        .collect::<Result<VecDeque<usize>>>()?;
    let player_2 = player_2_str
        .ok_or_else(|| anyhow!("No input for player 2"))?
        .lines()
        .skip(1)
        .map(|l| Ok(l.parse()?))
        .collect::<Result<VecDeque<usize>>>()?;

    let part_a = combat(player_1.clone(), player_2.clone());
    let part_b = recursive_combat(player_1, player_2);
    Ok((part_a.score(), Some(part_b.score())))
}
