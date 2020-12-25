use anyhow::{anyhow, Result};
use std::path::Path;

use crate::reader::split_once;

const MOD: u64 = 20201227;

fn find_loop_size(key: u64, subject_number: u64) -> u64 {
    let mut x = 1;
    for i in 0.. {
        if x == key {
            return i;
        }
        x *= subject_number;
        x %= MOD;
    }
    panic!("Range exhausted but no key found");
}

fn extract_encryption_key(public_key: u64, other_loop_size: u64) -> u64 {
    let mut x = 1;
    for _ in 0..other_loop_size {
        x *= public_key;
        x %= MOD;
    }
    x
}

pub fn main(path: &Path) -> Result<(u64, Option<usize>)> {
    let input = std::fs::read_to_string(path)?;
    let (card_str, door_str) = split_once(&input, "\n");
    let card_public_key = card_str.parse::<u64>()?;
    let door_public_key = door_str
        .ok_or_else(|| anyhow!("Couldn't find door key"))?
        .trim_end()
        .parse::<u64>()?;

    let card_loop_size = find_loop_size(card_public_key, 7);
    let door_loop_size = find_loop_size(door_public_key, 7);

    let encryption_key = extract_encryption_key(card_public_key, door_loop_size);
    let alt_encryption_key = extract_encryption_key(door_public_key, card_loop_size);
    if encryption_key != alt_encryption_key {
        return Err(anyhow!(
            "The card and door's encryption keys don't match. Something is wrong"
        ));
    }

    // NOTE: There is no part B
    Ok((encryption_key, None))
}
