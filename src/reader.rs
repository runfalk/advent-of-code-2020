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

pub fn read_mapped_lines<P, F, T, E>(path: P, f: F) -> Result<impl Iterator<Item = Result<T>>>
where
    P: AsRef<Path>,
    F: 'static + Fn(&str) -> Result<T, E>,
    T: 'static,
    anyhow::Error: From<E>,
{
    Ok(read_lines(path)?
        .into_iter()
        .map(move |l| -> Result<T> { Ok(f(&l?)?) }))
}

pub fn read_parsed_lines<P, T>(path: P) -> Result<impl Iterator<Item = Result<T>>>
where
    P: AsRef<Path>,
    T: 'static + FromStr,
    anyhow::Error: From<T::Err>,
{
    read_mapped_lines(path, T::from_str)
}
