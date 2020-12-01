use anyhow::{anyhow, Result};
use itertools::Itertools;
use std::path::Path;

use crate::reader::read_lines;

/// Find num_entries in entries that sum to 2020 and return their product
fn find_product_2020(entries: &[usize], num_entries: usize) -> Result<usize> {
    for selection in entries.iter().combinations(num_entries) {
        // Check if the selection sums up to 2020
        if selection.iter().cloned().sum::<usize>() == 2020 {
            // Return the product of the selection
            return Ok(selection.into_iter().fold(1, |p, f| p * f));
        }
    }
    Err(anyhow!("No matching entries"))
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let entries = read_lines(path)?
        .into_iter()
        .map(|l| Ok(l?.parse()?))
        .collect::<Result<Vec<_>>>()?;
    Ok((
        find_product_2020(&entries, 2)?,
        Some(find_product_2020(&entries, 3)?),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_input() -> Result<()> {
        assert!(find_product_2020(&[1010], 2).is_err());
        Ok(())
    }

    #[test]
    fn test_tricky_input() -> Result<()> {
        assert_eq!(find_product_2020(&[1010, 2010, 10], 2)?, 20100);
        Ok(())
    }
}
