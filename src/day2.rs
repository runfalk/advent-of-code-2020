use anyhow::{anyhow, Error, Result};
use once_cell::sync::Lazy;
use regex::Regex;
use std::path::Path;
use std::str::FromStr;

use crate::reader::read_parsed_lines;

// Create the row parsing regex once only to save some performance
static ROW_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(\d+)-(\d+)\s+([a-z]):\s+(\S+)$").unwrap());

#[derive(Debug)]
struct PasswordEntry {
    first: usize,
    second: usize,
    letter: char,
    password: String,
}

impl PasswordEntry {
    fn has_valid_frequency(&self) -> bool {
        let num_matches = self.password.chars().filter(|c| *c == self.letter).count();
        (self.first..=self.second).contains(&num_matches)
    }

    fn has_valid_positions(&self) -> bool {
        let first = self.password.chars().nth(self.first - 1).unwrap();
        let second = self.password.chars().nth(self.second - 1).unwrap();
        (first == self.letter) ^ (second == self.letter)
    }
}

impl FromStr for PasswordEntry {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = ROW_RE
            .captures(s)
            .ok_or(anyhow!("String doesn't match policy with password"))?;
        Ok(Self {
            first: captures[1].parse()?,
            second: captures[2].parse()?,
            letter: captures[3].chars().next().unwrap(),
            password: captures[4].to_owned(),
        })
    }
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let entries = read_parsed_lines(path)?.collect::<Result<Vec<PasswordEntry>>>()?;
    Ok((
        entries.iter().filter(|e| e.has_valid_frequency()).count(),
        Some(entries.iter().filter(|e| e.has_valid_positions()).count()),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation() -> Result<()> {
        let p1 = PasswordEntry::from_str("1-3 a: abcde")?;
        assert!(p1.has_valid_frequency());
        assert!(p1.has_valid_positions());

        let p2 = PasswordEntry::from_str("1-3 b: cdefg")?;
        assert!(!p2.has_valid_frequency());
        assert!(!p2.has_valid_positions());

        let p3 = PasswordEntry::from_str("2-9 c: ccccccccc")?;
        assert!(p3.has_valid_frequency());
        assert!(!p3.has_valid_positions());
        Ok(())
    }
}
