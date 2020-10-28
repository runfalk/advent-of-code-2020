use anyhow::Result;

pub fn main(_args: &[String]) -> Result<(usize, Option<usize>)> {
    Ok((0, None))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() -> Result<()> {
        assert_eq!(main(&vec!["Hello".to_owned()])?, (0, None));
        Ok(())
    }
}
