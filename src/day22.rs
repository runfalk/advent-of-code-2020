use anyhow::{anyhow, Result};
use std::collections::VecDeque;
use std::path::Path;

use crate::reader::split_once;

fn part_a(mut player_1: VecDeque<usize>, mut player_2: VecDeque<usize>) -> usize {
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

    let winner = if !player_1.is_empty() {
        player_1
    } else {
        player_2
    };

    winner
        .into_iter()
        .rev()
        .enumerate()
        .map(|(i, v)| (i + 1) * v)
        .sum::<usize>()
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

    Ok((part_a(player_1, player_2), None))
}
