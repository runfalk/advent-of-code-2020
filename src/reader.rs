use anyhow::Result;
use std::collections::HashMap;
use std::convert::TryInto;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

use crate::coord::Coord;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub tiles: HashMap<Coord, char>,
}

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

/// Split the string at the given separator. If the separator is not found, the
/// second part of the tuple will be None.
pub fn split_once<'a>(s: &'a str, pat: &str) -> (&'a str, Option<&'a str>) {
    let del_len = pat.len();
    match s.find(pat) {
        Some(i) => (&s[..i], Some(&s[i + del_len..])),
        None => (s, None),
    }
}

impl Grid {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Grid> {
        let mut width = 0;
        let mut height = 0;
        let mut tiles = HashMap::new();

        for (y, line) in read_lines(path)?.enumerate() {
            // NOTE: This doesn't check if lines have different length
            width = 0;
            for (x, c) in line?.chars().enumerate() {
                tiles.insert(Coord::new(x.try_into()?, y.try_into()?), c);
                width += 1;
            }
            height += 1;
        }
        Ok(Self {
            width,
            height,
            tiles,
        })
    }
}
