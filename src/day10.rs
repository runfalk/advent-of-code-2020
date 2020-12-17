use anyhow::Result;
use std::collections::HashMap;
use std::path::Path;

use crate::reader::read_parsed_lines;

fn solve(mut input: Vec<usize>) -> (usize, Option<u128>) {
    // Add charging outlet and built-in adapter
    input.push(0);
    input.sort_unstable();
    let built_in_adapter = input.last().unwrap() + 3;
    input.push(built_in_adapter);

    // Find number of one and three jolt differences
    let mut num_one_jolt = 0;
    let mut num_three_jolt = 0;
    for (prev, curr) in input.iter().zip(&input[1..]) {
        if curr - prev == 1 {
            num_one_jolt += 1;
        }
        if curr - prev == 3 {
            num_three_jolt += 1;
        }
    }

    // Find number of valid adapter configurations for each adapter
    let mut valid_paths = HashMap::new();
    valid_paths.insert(0, 1u128);
    for v in input[1..].iter() {
        // Check the previous 3 adapter jolt values for the number of valid paths.
        // The sum of those paths are the number of valid paths to this adapter
        let num_paths = (1..=3)
            .into_iter()
            .filter_map(|d| v.checked_sub(d).and_then(|p| valid_paths.get(&p)))
            .sum();
        valid_paths.insert(*v, num_paths);
    }

    (
        num_one_jolt * num_three_jolt,
        Some(valid_paths[&built_in_adapter]),
    )
}

pub fn main(path: &Path) -> Result<(usize, Option<u128>)> {
    let input = read_parsed_lines(path)?.collect::<Result<Vec<usize>>>()?;
    Ok(solve(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let input: Vec<usize> = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        assert_eq!(solve(input), (35, Some(8)));
    }

    #[test]
    fn test_intermediate() {
        let input: Vec<usize> = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        assert_eq!(solve(input), (220, Some(19208)));
    }
}
