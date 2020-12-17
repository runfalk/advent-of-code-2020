use anyhow::{anyhow, Result};
use itertools::Itertools;
use std::path::Path;

use crate::reader::read_parsed_lines;

fn find_first_weak_number(input: &[u64], preamble_len: usize) -> Option<u64> {
    for i in 0..input.len() - preamble_len {
        let curr = input[i + preamble_len];
        let is_weak_number = input[i..i + preamble_len]
            .iter()
            .tuple_combinations()
            .all(|(&a, &b)| a + b != curr);
        if is_weak_number {
            return Some(curr);
        }
    }
    None
}

fn find_encryption_weakness(input: &[u64], weak_number: u64) -> Option<u64> {
    for window_size in 2..input.len() {
        for window in input.windows(window_size) {
            if weak_number == window.iter().sum() {
                return window.iter().minmax().into_option().map(|(&a, &b)| a + b);
            }
        }
    }
    None
}

pub fn main(path: &Path) -> Result<(u64, Option<u64>)> {
    let preamble_len = 25usize;
    let input = read_parsed_lines(path)?.collect::<Result<Vec<u64>>>()?;
    let weak_number = find_first_weak_number(&input, preamble_len)
        .ok_or_else(|| anyhow!("Unable to find a weak number in the given input"))?;
    Ok((weak_number, find_encryption_weakness(&input, weak_number)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input: Vec<u64> = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];
        assert_eq!(find_first_weak_number(&input, 5), Some(127));
        assert_eq!(find_encryption_weakness(&input, 127), Some(62));
    }
}
