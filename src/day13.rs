use anyhow::{anyhow, Result};
use std::path::Path;

use crate::reader::read_lines;

fn part_a(bus_ids: &[(u64, u64)], departure_time: u64) -> Result<u64> {
    let (delay, bus_id) = bus_ids
        .iter()
        .map(|(_, bus_id)| (bus_id - (departure_time % bus_id), bus_id))
        .min()
        .ok_or_else(|| anyhow!("No buses"))?;
    Ok(bus_id * delay)
}

fn part_b(bus_ids: &[(u64, u64)]) -> u64 {
    // In the example we are looking for `t` such that the following equations
    // are satisfied:
    //
    // (t + 0) % 19 = 0
    // (t + 1) % 13 = 0
    // (t + 4) % 59 = 0
    // (t + 6) % 31 = 0
    // (t + 7) % 19 = 0
    //
    // These can be fed into something like Wolfram Alpha and it will give the
    // correct solution. (`t` is defined as `kx + m` and where `x` is all
    // integers, and the correct value is `m`).
    //
    // However, this Rust solution is a bit different (and is only correct
    // because the bus IDs are prime, though technically coprime is enough I
    // think).
    //
    // We start by setting the `cycle_time` to the `bus_id` of the first bus.
    // Then we add the second bus and look for the first `cycle_time` multiple
    // plus the delay where the second bus arrives. Since the `bus_id` is prime
    // the new `cycle_time` is `cycle_time * bus_id`. Every time we check if
    // the next bus arrives on time we increase `t` by `cycle_time`, as there
    // is no way it can happen at any other `t`. This is again because of that
    // the bus IDs are prime. The same process is repeated for the remaining
    // buses.
    let mut t = 0;
    let mut cycle_time = 1;
    for (delay, bus_id) in bus_ids {
        while (t + delay) % bus_id != 0 {
            t += cycle_time;
        }
        cycle_time *= bus_id;
    }
    t
}

pub fn main(path: &Path) -> Result<(u64, Option<u64>)> {
    let mut lines = read_lines(path)?;
    let departure_time: u64 = lines
        .next()
        .transpose()?
        .ok_or_else(|| anyhow!("Unable to read first line of input"))?
        .parse()?;
    let bus_ids = lines
        .next()
        .transpose()?
        .ok_or_else(|| anyhow!("Unable to read second line of input"))?
        .split(',')
        .enumerate()
        .filter_map(|(i, id)| Some((i as u64, id.parse().ok()?)))
        .collect::<Vec<(u64, u64)>>();
    Ok((part_a(&bus_ids, departure_time)?, Some(part_b(&bus_ids))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> Result<()> {
        let bus_ids = vec![(0, 7), (1, 13), (4, 59), (6, 31), (7, 19)];
        assert_eq!(part_a(&bus_ids, 939)?, 295);
        assert_eq!(part_b(&bus_ids), 1068781);
        Ok(())
    }
}
