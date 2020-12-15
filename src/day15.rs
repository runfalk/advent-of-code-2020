use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::path::Path;
use std::str::FromStr;

use crate::reader::read_parsed_lines;

fn find_nth_num(starting_numbers: &[usize], n: usize) -> usize {
    if n <= starting_numbers.len() {
        return starting_numbers[n - 1];
    }

    let mut last_spoken = HashMap::new();
    for (i, v) in starting_numbers.into_iter().enumerate() {
        last_spoken.insert(*v, i);
    }

    let mut next_num = 0;
    for i in starting_numbers.len()..n - 1 {
        let ne = match last_spoken.get(&next_num) {
            Some(n) => i - n,
            None => 0,
        };
        last_spoken.insert(next_num, i);
        next_num = ne;
    }
    next_num
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let starting_numbers = std::fs::read_to_string(path)?
        .trim_end()
        .split(",")
        .map(|i| Ok(i.parse::<usize>()?))
        .collect::<Result<Vec<_>>>()?;

    Ok((
        find_nth_num(&starting_numbers, 2020),
        Some(find_nth_num(&starting_numbers, 30000000)),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_example() {
        let first = vec![0, 3, 6];
        assert_eq!(find_nth_num(&first, 1), 0);
        assert_eq!(find_nth_num(&first, 2), 3);
        assert_eq!(find_nth_num(&first, 3), 6);
        assert_eq!(find_nth_num(&first, 4), 0);
        assert_eq!(find_nth_num(&first, 5), 3);
        assert_eq!(find_nth_num(&first, 6), 3);
        assert_eq!(find_nth_num(&first, 7), 1);
        assert_eq!(find_nth_num(&first, 8), 0);
        assert_eq!(find_nth_num(&first, 9), 4);
        assert_eq!(find_nth_num(&first, 10), 0);
        assert_eq!(find_nth_num(&first, 2020), 436);
    }

    #[test]
    fn test_more_examples() {
        assert_eq!(find_nth_num(&[1, 3, 2], 2020), 1);
        assert_eq!(find_nth_num(&[2, 1, 3], 2020), 10);
        assert_eq!(find_nth_num(&[1, 2, 3], 2020), 27);
        assert_eq!(find_nth_num(&[2, 3, 1], 2020), 78);
        assert_eq!(find_nth_num(&[3, 2, 1], 2020), 438);
        assert_eq!(find_nth_num(&[3, 1, 2], 2020), 1836);
    }

    #[test]
    #[ignore]
    fn test_long_examples() {
        assert_eq!(find_nth_num(&[0, 3, 6], 30_000_000), 175594);
        assert_eq!(find_nth_num(&[1, 3, 2], 30_000_000), 2578);
        assert_eq!(find_nth_num(&[2, 1, 3], 30_000_000), 3544142);
        assert_eq!(find_nth_num(&[1, 2, 3], 30_000_000), 261214);
        assert_eq!(find_nth_num(&[2, 3, 1], 30_000_000), 6895259);
        assert_eq!(find_nth_num(&[3, 2, 1], 30_000_000), 18);
        assert_eq!(find_nth_num(&[3, 1, 2], 30_000_000), 362);
    }
}
