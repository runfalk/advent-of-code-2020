use anyhow::{anyhow, Result};
use std::collections::HashSet;
use std::convert::TryInto;
use std::path::Path;

use crate::coord::Coord;
use crate::reader::read_lines;

#[derive(Debug)]
struct Map {
    width: usize,
    height: usize,
    trees: HashSet<Coord>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoordJumps {
    origin: Coord,
    x: isize,
    y: isize,
    i: isize,
}

impl Map {
    fn from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut width = 0;
        let mut height = 0;
        let mut trees = HashSet::new();

        for (y, line) in read_lines(path)?.enumerate() {
            for (x, c) in line?.chars().enumerate() {
                match c {
                    '#' => {
                        trees.insert(Coord::new(x.try_into()?, y.try_into()?));
                    }
                    '.' => (),
                    _ => return Err(anyhow!("Invalid character encountered")),
                };
                width = x + 1;
            }
            height = y + 1;
        }

        Ok(Self {
            width,
            height,
            trees,
        })
    }

    fn is_tree(&self, coord: &Coord) -> bool {
        let x = coord.x % self.width as isize;
        self.trees.contains(&Coord::new(x, coord.y))
    }

    fn in_bounds(&self, coord: &Coord) -> bool {
        coord.y >= 0 && coord.y < self.height as isize
    }
}

impl CoordJumps {
    fn new(origin: Coord, x: isize, y: isize) -> Self {
        Self { origin, x, y, i: 0 }
    }
}

impl Iterator for CoordJumps {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        let output = Some(Coord::new(
            self.origin.x + self.x * self.i,
            self.origin.y + self.y * self.i,
        ));
        self.i += 1;
        output
    }
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let map = Map::from_path(path)?;
    let part_a = CoordJumps::new(Coord::origin(), 3, 1)
        .take_while(|c| map.in_bounds(c))
        .filter(|c| map.is_tree(c))
        .count();

    let mut part_b = part_a;
    for (r, d) in vec![(1, 1), (5, 1), (7, 1), (1, 2)] {
        part_b *= CoordJumps::new(Coord::origin(), r, d)
            .take_while(|c| map.in_bounds(c))
            .filter(|c| map.is_tree(c))
            .count();
    }

    Ok((part_a, Some(part_b)))
}
