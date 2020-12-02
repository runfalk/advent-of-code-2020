use anyhow::Result;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

pub fn read_lines<P>(filename: P) -> Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    Ok(io::BufReader::new(File::open(filename)?).lines())
}

pub fn read_parsed_lines<P, T>(filename: P) -> Result<impl Iterator<Item = Result<T>>>
where
    P: AsRef<Path>,
    T: FromStr,
    anyhow::Error: From<T::Err>,
{
    Ok(read_lines(filename)?
        .into_iter()
        .map(|l| -> Result<T> { Ok(l?.parse()?) }))
}
