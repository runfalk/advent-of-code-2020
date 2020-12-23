use anyhow::{anyhow, Result};
use std::path::Path;

fn encode(s: &str) -> Result<(usize, usize, Vec<usize>)> {
    let input = s
        .chars()
        .map(|c| {
            c.to_digit(10)
                .map(|d| d as usize - 1)
                .ok_or_else(|| anyhow!("Invalid character {:?}", c))
        })
        .collect::<Result<Vec<_>>>()?;

    let starting_cup = input[0];
    let last_cup = *input.last().unwrap();

    let mut cups = vec![0; input.len()];
    for (cup, next) in input
        .iter()
        .copied()
        .zip(input.iter().cycle().skip(1).copied())
    {
        cups[cup] = next;
    }

    Ok((starting_cup, last_cup, cups))
}

fn decode(slice: &[usize], from: usize, n: usize) -> Vec<usize> {
    let mut out = Vec::new();
    let mut next = from - 1;
    for _ in 0..n {
        out.push(next + 1);
        next = slice[next];
    }
    out
}

fn crab_game(mut cups: Vec<usize>, starting_cup: usize, iterations: usize) -> Vec<usize> {
    let mut next_cup = starting_cup;
    for _ in 0..iterations {
        let mut target_cup = next_cup.checked_sub(1).unwrap_or(cups.len() - 1);
        let a = cups[next_cup];
        let b = cups[a];
        let c = cups[b];

        while target_cup == a || target_cup == b || target_cup == c {
            target_cup = target_cup.checked_sub(1).unwrap_or(cups.len() - 1);
        }

        cups[next_cup] = cups[c];
        next_cup = cups[c];
        cups[c] = cups[target_cup];
        cups[target_cup] = a;
    }
    cups
}

pub fn main(path: &Path) -> Result<(String, Option<u64>)> {
    let (starting_cup, last_cup, mut cups) = encode(std::fs::read_to_string(path)?.trim_end())?;

    let first_game = crab_game(cups.clone(), starting_cup, 100);

    cups.extend(10..=1_000_000);
    cups[last_cup] = 9;
    *cups.last_mut().unwrap() = starting_cup;
    let second_game = crab_game(cups, starting_cup, 10_000_000);

    Ok((
        decode(&first_game, 1, 9)
            .into_iter()
            .skip(1)
            .map(|c| c.to_string())
            .collect(),
        Some(
            decode(&second_game, 1, 3)
                .into_iter()
                .skip(1)
                .map(|c| c as u64)
                .product(),
        ),
    ))
}
