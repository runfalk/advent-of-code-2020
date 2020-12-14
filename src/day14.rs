use anyhow::{anyhow, Result};
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;
use std::path::Path;
use std::str::FromStr;

use crate::reader::read_parsed_lines;

static ACTION_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(mask|mem)(?:\[(\d+)\])? = ([01X]+|\d+)$").unwrap());

#[derive(Debug)]
enum Action {
    Mask(Vec<Option<bool>>),
    Memset(usize, u64),
}

impl FromStr for Action {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let c = ACTION_RE.captures(s).ok_or(anyhow!("Invalid action"))?;
        Ok(match &c[1] {
            "mask" => Action::Mask(
                c[3].chars()
                    .map(|c| match c {
                        '0' => Some(false),
                        '1' => Some(true),
                        'X' => None,
                        _ => unreachable!(),
                    })
                    .collect(),
            ),
            "mem" => Action::Memset(c[2].parse()?, c[3].parse()?),
            _ => unreachable!(),
        })
    }
}

fn mask_addr(addr: usize, mask: &[Option<bool>]) -> String {
    format!("{:036b}", addr)
        .chars()
        .zip(mask.iter())
        .map(|(a, &m)| match m {
            Some(false) => a,
            Some(true) => '1',
            None => 'X',
        })
        .collect()
}

fn replace_wildcards(masked_addr: &str) -> Vec<String> {
    let mut v = Vec::new();
    if masked_addr.find("X").is_some() {
        v.extend(replace_wildcards(&masked_addr.replacen("X", "0", 1)));
        v.extend(replace_wildcards(&masked_addr.replacen("X", "1", 1)));
    } else {
        v.push(masked_addr.to_owned());
    }
    v
}

fn part_a(actions: &[Action]) -> u64 {
    let mut memory = HashMap::new();
    let mut mask = [].as_ref();
    for action in actions {
        match action {
            Action::Mask(ref m) => mask = m.as_slice(),
            Action::Memset(address, mut value) => {
                for (i, bit) in mask
                    .iter()
                    .rev()
                    .enumerate()
                    .filter_map(|(i, m)| Some((i, m.as_ref()?)))
                {
                    if *bit {
                        value |= 1 << i;
                    } else {
                        value &= !0 ^ (1 << i);
                    }
                }
                memory.insert(address, value);
            }
        }
    }
    memory.values().map(|v| *v).sum()
}

fn part_b(actions: &[Action]) -> u64 {
    let mut memory = HashMap::new();
    let mut mask = [].as_ref();
    for action in actions {
        match action {
            Action::Mask(m) => mask = m,
            Action::Memset(address, value) => {
                let masked_addr = mask_addr(*address, mask);
                for masked_addr in replace_wildcards(&masked_addr) {
                    memory.insert(masked_addr, value);
                }
            }
        }
    }
    memory.values().map(|v| *v).sum()
}

pub fn main(path: &Path) -> Result<(u64, Option<u64>)> {
    let actions = read_parsed_lines(path)?.collect::<Result<Vec<Action>>>()?;
    Ok((part_a(&actions), Some(part_b(&actions))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a() -> Result<()> {
        let actions = vec![
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X",
            "mem[8] = 11",
            "mem[7] = 101",
            "mem[8] = 0",
        ]
        .into_iter()
        .map(Action::from_str)
        .collect::<Result<Vec<Action>>>()?;
        assert_eq!(part_a(&actions), 165);
        Ok(())
    }

    #[test]
    fn test_part_b() -> Result<()> {
        let actions = vec![
            "mask = 000000000000000000000000000000X1001X",
            "mem[42] = 100",
            "mask = 00000000000000000000000000000000X0XX",
            "mem[26] = 1",
        ]
        .into_iter()
        .map(Action::from_str)
        .collect::<Result<Vec<Action>>>()?;
        assert_eq!(part_b(&actions), 208);
        Ok(())
    }
}
