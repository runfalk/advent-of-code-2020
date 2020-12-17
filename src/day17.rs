use anyhow::Result;
use itertools::iproduct;
use std::collections::HashSet;
use std::path::Path;

use crate::reader::read_lines;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Coord3d {
    x: isize,
    y: isize,
    z: isize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Coord4d {
    x: isize,
    y: isize,
    z: isize,
    w: isize,
}

trait Coord: Sized {
    fn num_neighbors(&self, cubes: &HashSet<Self>) -> usize;
    fn min_max_bounds(cubes: &HashSet<Self>) -> Option<(Self, Self)>;
    // Box workaround because impl Trait is not supported as a return type for trait methods
    fn iter(a: &Self, b: &Self) -> Box<dyn Iterator<Item = Self>>;
}

impl Coord3d {
    fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }
}

impl Coord4d {
    fn new(x: isize, y: isize, z: isize, w: isize) -> Self {
        Self { x, y, z, w }
    }
}

impl Coord for Coord3d {
    fn num_neighbors(&self, cubes: &HashSet<Self>) -> usize {
        iproduct!(-1..=1, -1..=1, -1..=1)
            .map(|(dx, dy, dz)| Self::new(self.x + dx, self.y + dy, self.z + dz))
            .filter(|c| c != self && cubes.contains(c))
            .count()
    }

    fn min_max_bounds(cubes: &HashSet<Self>) -> Option<(Self, Self)> {
        if cubes.len() == 0 {
            return None;
        }

        let (mut min_x, mut max_x) = (isize::MAX, isize::MIN);
        let (mut min_y, mut max_y) = (isize::MAX, isize::MIN);
        let (mut min_z, mut max_z) = (isize::MAX, isize::MIN);

        for c in cubes {
            min_x = min_x.min(c.x);
            max_x = max_x.max(c.x);
            min_y = min_y.min(c.y);
            max_y = max_y.max(c.y);
            min_z = min_z.min(c.z);
            max_z = max_z.max(c.z);
        }

        Some((
            Self::new(min_x - 1, min_y - 1, min_z - 1),
            Self::new(max_x + 1, max_y + 1, max_z + 1),
        ))
    }

    fn iter(a: &Self, b: &Self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(iproduct!(a.x..=b.x, a.y..=b.y, a.z..=b.z).map(|(x, y, z)| Self::new(x, y, z)))
    }
}

impl Coord for Coord4d {
    fn num_neighbors(&self, cubes: &HashSet<Self>) -> usize {
        iproduct!(-1..=1, -1..=1, -1..=1, -1..=1)
            .map(|(dx, dy, dz, dw)| Self::new(self.x + dx, self.y + dy, self.z + dz, self.w + dw))
            .filter(|c| c != self && cubes.contains(c))
            .count()
    }

    fn min_max_bounds(cubes: &HashSet<Self>) -> Option<(Self, Self)> {
        if cubes.len() == 0 {
            return None;
        }

        let (mut min_x, mut max_x) = (isize::MAX, isize::MIN);
        let (mut min_y, mut max_y) = (isize::MAX, isize::MIN);
        let (mut min_z, mut max_z) = (isize::MAX, isize::MIN);
        let (mut min_w, mut max_w) = (isize::MAX, isize::MIN);

        for c in cubes {
            min_x = min_x.min(c.x);
            max_x = max_x.max(c.x);
            min_y = min_y.min(c.y);
            max_y = max_y.max(c.y);
            min_z = min_z.min(c.z);
            max_z = max_z.max(c.z);
            min_w = min_w.min(c.w);
            max_w = max_w.max(c.w);
        }

        Some((
            Self::new(min_x - 1, min_y - 1, min_z - 1, min_w - 1),
            Self::new(max_x + 1, max_y + 1, max_z + 1, max_w + 1),
        ))
    }

    fn iter(a: &Self, b: &Self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            iproduct!(a.x..=b.x, a.y..=b.y, a.z..=b.z, a.w..=b.w)
                .map(|(x, y, z, w)| Self::new(x, y, z, w)),
        )
    }
}

fn num_cubes_at_cycle<T>(start: HashSet<T>, n: usize) -> usize
where
    T: Coord + Eq + std::hash::Hash,
{
    let mut prev = start;
    for _ in 0..n {
        let mut curr = HashSet::new();
        let (min, max) = if let Some((min, max)) = T::min_max_bounds(&prev) {
            (min, max)
        } else {
            return 0;
        };
        for c in T::iter(&min, &max) {
            let num_neighbors = c.num_neighbors(&prev);
            if num_neighbors == 3 || prev.contains(&c) && num_neighbors == 2 {
                curr.insert(c);
            }
        }
        prev = curr;
    }
    prev.len()
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let mut cubes_3d = HashSet::new();
    let mut cubes_4d = HashSet::new();
    for (y, l) in read_lines(path)?.enumerate() {
        for (x, c) in l?.chars().enumerate() {
            if c == '#' {
                cubes_3d.insert(Coord3d::new(x as isize, y as isize, 0));
                cubes_4d.insert(Coord4d::new(x as isize, y as isize, 0, 0));
            }
        }
    }
    Ok((
        num_cubes_at_cycle(cubes_3d, 6),
        Some(num_cubes_at_cycle(cubes_4d, 6)),
    ))
}
