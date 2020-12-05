use anyhow::{anyhow, Result};
use std::path::Path;

use crate::reader::read_mapped_lines;

fn seat_spec_to_id(seat: &str) -> Result<usize> {
    // NOTE: We don't validate the order and length of the seat number
    let bin_repr = seat
        .chars()
        .map(|c| match c {
            'F' | 'L' => Ok('0'),
            'B' | 'R' => Ok('1'),
            e => Err(anyhow!("Invalid character {:?} in seat number", e)),
        })
        .collect::<Result<String>>()?;
    Ok(usize::from_str_radix(&bin_repr, 2)?)
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let mut seat_ids = read_mapped_lines(path, seat_spec_to_id)?;
    let mut min = seat_ids.next().transpose()?.ok_or(anyhow!("No seat IDs"))?;
    let mut max = min;
    let mut sum = min;

    for seat_id in seat_ids {
        let seat_id = seat_id?;
        if seat_id < min {
            min = seat_id;
        } else if seat_id > max {
            max = seat_id;
        }
        sum += seat_id;
    }
    Ok((max, Some((min..=max).into_iter().sum::<usize>() - sum)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seat_spec_to_id() -> Result<()> {
        assert_eq!(seat_spec_to_id("BFFFBBFRRR")?, 567);
        assert_eq!(seat_spec_to_id("FFFBBBFRRR")?, 119);
        assert_eq!(seat_spec_to_id("BBFFBBFRLL")?, 820);
        Ok(())
    }
}
