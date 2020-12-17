use anyhow::Result;
use std::path::Path;

use crate::coord::Coord;
use crate::reader::Grid;

fn is_tree(map: &Grid, coord: &Coord) -> bool {
    let x = coord.x % map.width as isize;
    map.tiles.get(&Coord::new(x, coord.y)) == Some(&'#')
}

fn is_in_bounds(map: &Grid, coord: &Coord) -> bool {
    coord.y >= 0 && coord.y < map.height as isize
}

fn num_trees_encountered(map: &Grid, x_step: isize, y_step: isize) -> usize {
    (1..)
        .into_iter()
        .map(|i| Coord::new(i * x_step, i * y_step))
        .take_while(|c| is_in_bounds(map, c))
        .filter(|c| is_tree(map, c))
        .count()
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let map = Grid::from_path(path)?;
    let part_a = num_trees_encountered(&map, 3, 1);

    let mut part_b = part_a;
    for (x_step, y_step) in &[(1, 1), (5, 1), (7, 1), (1, 2)] {
        part_b *= num_trees_encountered(&map, *x_step, *y_step);
    }

    Ok((part_a, Some(part_b)))
}
