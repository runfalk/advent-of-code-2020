use anyhow::{anyhow, Result};
use std::path::Path;

use crate::coord::Coord;
use crate::reader::read_lines;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Floor,
    Empty,
    Occupied,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Layout {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
}

impl Layout {
    fn coord_to_index(&self, c: &Coord) -> Option<usize> {
        if c.x < 0 || c.y < 0 || c.x >= self.width as isize || c.y >= self.height as isize {
            return None;
        }
        Some(c.y as usize * self.width + c.x as usize % self.width)
    }

    fn get(&self, c: &Coord) -> Option<Tile> {
        let i = self.coord_to_index(c)?;
        self.tiles.get(i).copied()
    }
}

impl Tile {
    fn from_char(c: char) -> Result<Self> {
        Ok(match c {
            '.' => Self::Floor,
            'L' => Self::Empty,
            '#' => Self::Occupied,
            _ => return Err(anyhow!("Invalid character {}", c)),
        })
    }
}

fn run_until_steady_state<F>(
    mut layout: Layout,
    max_neighbors: usize,
    num_neighbors: F,
) -> Result<usize>
where
    F: Fn(&Layout, &Coord) -> usize,
{
    let mut new_tiles = Vec::with_capacity(layout.tiles.len());
    loop {
        for y in 0..layout.height {
            for x in 0..layout.width {
                let c = Coord::new(x as isize, y as isize);
                let tile = layout
                    .get(&c)
                    .ok_or_else(|| anyhow!("Coordinate {:?} not in layout", c))?;
                let num_neighbors = num_neighbors(&layout, &c);
                new_tiles.push(match tile {
                    Tile::Floor => Tile::Floor,
                    Tile::Empty => {
                        if num_neighbors == 0 {
                            Tile::Occupied
                        } else {
                            Tile::Empty
                        }
                    }
                    Tile::Occupied => {
                        if num_neighbors >= max_neighbors {
                            Tile::Empty
                        } else {
                            Tile::Occupied
                        }
                    }
                });
            }
        }
        if new_tiles == layout.tiles {
            break;
        }
        std::mem::swap(&mut layout.tiles, &mut new_tiles);
        new_tiles.clear();
    }
    Ok(layout
        .tiles
        .into_iter()
        .filter(|t| *t == Tile::Occupied)
        .count())
}

fn part_a(layout: Layout) -> Result<usize> {
    run_until_steady_state(layout, 4, |layout, coord| {
        coord
            .iter_all_neighbors()
            .filter(|n| layout.get(&n) == Some(Tile::Occupied))
            .count()
    })
}

fn part_b(layout: Layout) -> Result<usize> {
    run_until_steady_state(layout, 5, |layout, coord| {
        Coord::origin()
            .iter_all_neighbors()
            .filter(|d| {
                for i in 1.. {
                    let ray = Coord::new(coord.x + d.x * i, coord.y + d.y * i);
                    match layout.get(&ray) {
                        Some(Tile::Floor) => {}
                        Some(Tile::Occupied) => {
                            return true;
                        }
                        Some(Tile::Empty) | None => {
                            return false;
                        }
                    };
                }
                unreachable!();
            })
            .count()
    })
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let mut tiles = Vec::new();
    let mut height = 0;
    for line in read_lines(path)? {
        for c in line?.chars() {
            tiles.push(Tile::from_char(c)?);
        }
        height += 1;
    }
    let layout = Layout {
        width: tiles.len() / height,
        height,
        tiles,
    };
    Ok((part_a(layout.clone())?, Some(part_b(layout)?)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> Result<()> {
        let layout = Layout {
            width: 10,
            height: 10,
            tiles: vec![
                "L.LL.LL.LL",
                "LLLLLLL.LL",
                "L.L.L..L..",
                "LLLL.LL.LL",
                "L.LL.LL.LL",
                "L.LLLLL.LL",
                "..L.L.....",
                "LLLLLLLLLL",
                "L.LLLLLL.L",
                "L.LLLLL.LL",
            ]
            .into_iter()
            .map(|l| l.chars().map(|c| Tile::from_char(c).unwrap()))
            .flatten()
            .collect(),
        };
        assert_eq!(part_a(layout.clone())?, 37);
        assert_eq!(part_b(layout)?, 26);
        Ok(())
    }
}
