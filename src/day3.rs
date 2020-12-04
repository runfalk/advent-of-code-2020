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

fn num_trees_encountered(map: &Map, x_step: isize, y_step: isize) -> usize {
    (1..)
        .into_iter()
        .map(|i| Coord::new(i * x_step, i * y_step))
        .take_while(|c| map.in_bounds(c))
        .filter(|c| map.is_tree(c))
        .count()
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let map = Map::from_path(path)?;
    let part_a = num_trees_encountered(&map, 3, 1);

    let mut part_b = part_a;
    for (x_step, y_step) in vec![(1, 1), (5, 1), (7, 1), (1, 2)] {
        part_b *= num_trees_encountered(&map, x_step, y_step);
    }

    Ok((part_a, Some(part_b)))
}
