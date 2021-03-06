use anyhow::{anyhow, Result};
use std::collections::{HashMap, HashSet};
use std::path::Path;

use crate::reader::read_mapped_lines;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct HexCoord {
    x: isize,
    y: isize,
    z: isize,
}

#[derive(Debug, PartialEq, Eq)]
enum Step {
    Nw,
    Ne,
    E,
    Se,
    Sw,
    W,
}

impl HexCoord {
    fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }

    fn origin() -> Self {
        Self::new(0, 0, 0)
    }

    fn step(&mut self, step: &Step) {
        let (dx, dy, dz) = match step {
            Step::Nw => (0, 1, -1),
            Step::Ne => (1, 0, -1),
            Step::E => (1, -1, 0),
            Step::Se => (0, -1, 1),
            Step::Sw => (-1, 0, 1),
            Step::W => (-1, 1, 0),
        };
        self.x += dx;
        self.y += dy;
        self.z += dz;
    }

    fn follow_steps(&mut self, steps: &[Step]) {
        for step in steps {
            self.step(step);
        }
    }

    fn neighbors(&self) -> impl Iterator<Item = Self> {
        let origin = self.clone();
        vec![
            (0, 1, -1),
            (1, 0, -1),
            (1, -1, 0),
            (0, -1, 1),
            (-1, 0, 1),
            (-1, 1, 0),
        ]
        .into_iter()
        .map(move |(dx, dy, dz)| Self::new(origin.x + dx, origin.y + dy, origin.z + dz))
    }
}

fn parse_steps(s: &str) -> Result<Vec<Step>> {
    let mut out = Vec::new();
    let chars = s.chars().collect::<Vec<_>>();
    let mut it = 0..s.len();
    while let Some(i) = it.next() {
        let step = match (chars[i], chars.get(i + 1).copied()) {
            ('n', Some('w')) => {
                it.next();
                Step::Nw
            }
            ('n', Some('e')) => {
                it.next();
                Step::Ne
            }
            ('e', _) => Step::E,
            ('s', Some('e')) => {
                it.next();
                Step::Se
            }
            ('s', Some('w')) => {
                it.next();
                Step::Sw
            }
            ('w', _) => Step::W,
            (c, _) => return Err(anyhow!("Unexpected direction {:?}", c)),
        };
        out.push(step);
    }
    Ok(out)
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let tile_paths = read_mapped_lines(path, parse_steps)?.collect::<Result<Vec<_>>>()?;

    let mut black_tiles = HashSet::new();
    for tp in tile_paths {
        let mut tile = HexCoord::origin();
        tile.follow_steps(&tp);

        if black_tiles.contains(&tile) {
            black_tiles.remove(&tile);
        } else {
            black_tiles.insert(tile);
        }
    }
    let part_a = black_tiles.len();

    let mut today = black_tiles;
    for _ in 0..100 {
        let mut tomorrow = HashSet::new();
        let mut black_neighbors: HashMap<HexCoord, usize> = HashMap::new();
        for tile in today.iter() {
            for n in tile.neighbors() {
                *black_neighbors.entry(n).or_default() += 1;
            }
        }

        for (tile, num_neighbors) in black_neighbors {
            if (today.contains(&tile) && num_neighbors == 1) || num_neighbors == 2 {
                tomorrow.insert(tile);
            }
        }

        today = tomorrow;
    }
    let part_b = today.len();

    Ok((part_a, Some(part_b)))
}
