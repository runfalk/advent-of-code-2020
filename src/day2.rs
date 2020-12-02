use anyhow::{anyhow, Error, Result};
use regex::Regex;
use std::path::Path;
use std::str::FromStr;

use crate::reader::read_parsed_lines;

#[derive(Debug)]
struct Policy {
    needle: char,
    first: usize,
    second: usize,
}

impl Policy {
    fn is_valid_part1(&self, password: &str) -> bool {
        let num_matches = password.chars().filter(|c| *c == self.needle).count();
        (self.first..=self.second).contains(&num_matches)
    }

    fn is_valid_part2(&self, password: &str) -> bool {
        let first = password.chars().nth(self.first - 1).unwrap();
        let second = password.chars().nth(self.second - 1).unwrap();
        (first == self.needle) ^ (second == self.needle)
    }
}

#[derive(Debug)]
struct PolicyWithPassword {
    policy: Policy,
    password: String,
}

impl FromStr for PolicyWithPassword {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^(\d+)-(\d+)\s+([a-z]):\s+(\S+)$")?;
        let captures = re
            .captures(s)
            .ok_or(anyhow!("String doesn't match policy with password"))?;
        Ok(Self {
            policy: Policy {
                needle: captures[3].chars().next().unwrap(),
                first: captures[1].parse()?,
                second: captures[2].parse()?,
            },
            password: captures[4].to_owned(),
        })
    }
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let entries = read_parsed_lines(path)?.collect::<Result<Vec<PolicyWithPassword>>>()?;
    Ok((
        entries
            .iter()
            .filter(|e| e.policy.is_valid_part1(&e.password))
            .count(),
        Some(
            entries
                .iter()
                .filter(|e| e.policy.is_valid_part2(&e.password))
                .count(),
        ),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation() -> Result<()> {
        let p1 = PolicyWithPassword::from_str("1-3 a: abcde")?;
        assert!(p1.policy.is_valid_part1(&p1.password));
        assert!(p1.policy.is_valid_part2(&p1.password));

        let p2 = PolicyWithPassword::from_str("1-3 b: cdefg")?;
        assert!(!p2.policy.is_valid_part1(&p2.password));
        assert!(!p2.policy.is_valid_part2(&p2.password));

        let p3 = PolicyWithPassword::from_str("2-9 c: ccccccccc")?;
        assert!(p3.policy.is_valid_part1(&p3.password));
        assert!(!p3.policy.is_valid_part2(&p3.password));
        Ok(())
    }
}
