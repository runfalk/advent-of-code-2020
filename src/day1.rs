use anyhow::Result;
use std::path::Path;

fn part_a() -> u32 {
    0
}

pub fn main(_path: &Path) -> Result<(usize, Option<usize>)> {
    Ok((0, None))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() -> Result<()> {
        assert_eq!(part_a(), 0);
        Ok(())
    }
}
