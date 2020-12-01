use anyhow::{anyhow, Result};
use itertools::iproduct;
use std::path::Path;

use crate::reader::read_lines;

fn part_a(entries: &[usize]) -> Result<usize> {
    for (x, y) in iproduct!(entries.iter(), entries.iter()) {
        if x + y == 2020 {
            return Ok(x * y);
        }
    }
    Err(anyhow!("No matching transactions"))
}

fn part_b(entries: &[usize]) -> Result<usize> {
    for (x, y, z) in iproduct!(entries.iter(), entries.iter(), entries.iter()) {
        if x + y + z == 2020 {
            return Ok(x * y * z);
        }
    }
    Err(anyhow!("No matching transactions"))
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let entries = read_lines(path)?
        .into_iter()
        .map(|l| Ok(l?.parse()?))
        .collect::<Result<Vec<_>>>()?;
    Ok((part_a(&entries)?, Some(part_b(&entries)?)))
}
