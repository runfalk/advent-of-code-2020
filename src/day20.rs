use anyhow::{anyhow, Result};
use std::collections::{HashMap, HashSet};
use std::ops::Deref;
use std::path::Path;

#[derive(Debug, Clone)]
struct Piece {
    top: Vec<bool>,
    left: Vec<bool>,
    right: Vec<bool>,
    bottom: Vec<bool>,
}

impl Piece {
    fn from_matrix<T: Deref<Target = [bool]>>(matrix: &[T]) -> Self {
        let top = matrix[0].to_vec();
        let bottom = matrix.last().unwrap().to_vec();

        let mut left = Vec::new();
        let mut right = Vec::new();
        for row in matrix {
            left.push(row[0]);
            right.push(*row.last().unwrap());
        }

        // NOTE: No sanity checks are done
        Self {
            top,
            left,
            right,
            bottom,
        }
    }

    /// Generate all possible orientations of a this piece
    fn all_orientations(self) -> Vec<Self> {
        // There are 8 different ways of transforming a piece. Any combination of these operations:
        // - Transposing
        // - Flipping horizontally
        // - Flipping vertically
        let mut orientations = Vec::new();
        for i in 0..8 {
            let transpose = (i >> 2) == 1;
            let flip_x = (i >> 1) & 1 == 1;
            let flip_y = i & 1 == 1;

            let Self {
                mut top,
                mut left,
                mut right,
                mut bottom,
            } = self.clone();

            if transpose {
                std::mem::swap(&mut top, &mut left);
                std::mem::swap(&mut right, &mut bottom);
            }

            if flip_x {
                top.reverse();
                bottom.reverse();
                std::mem::swap(&mut left, &mut right);
            }

            if flip_y {
                left.reverse();
                right.reverse();
                std::mem::swap(&mut top, &mut bottom);
            }

            orientations.push(Self {
                top,
                left,
                right,
                bottom,
            });
        }
        orientations
    }

    /// Returns a tuple that shows where this piece fits with the other one
    fn fits_with(&self, other: &Self) -> (bool, bool, bool, bool) {
        let top = self.top == other.bottom;
        let left = self.left == other.right;
        let right = self.right == other.left;
        let bottom = self.bottom == other.top;
        (top, left, right, bottom)
    }
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let entries = std::fs::read_to_string(path)?;

    let piece_matrices = entries
        .split("\n\n")
        .map(|piece| -> Result<_> {
            let mut lines = piece.lines();
            let id_str = lines.next().ok_or_else(|| anyhow!("No lines for piece"))?;
            let id: usize = id_str[5..id_str.len() - 1].parse()?;
            let matrix = lines
                .map(|line| line.chars().map(|c| c == '#').collect::<Vec<_>>())
                .collect::<Vec<_>>();
            Ok((id, matrix))
        })
        .collect::<Result<HashMap<_, _>>>()?;

    // Generate a mapping from tile ID to a list of all variations of that piece. We only store the
    // edges and provide some convenience methods to check if two variations fit together.
    let pieces = piece_matrices
        .iter()
        .map(|(id, matrix)| (*id, Piece::from_matrix(matrix).all_orientations()))
        .collect::<HashMap<_, _>>();

    // The image is square, so we can calculate the number of tiles per side in advance
    let side = (1..100).find(|i| i * i == pieces.len()).unwrap();

    // Construct a mapping from tile ID and variant ID (0-7) to four sets representing all piece
    // variants the current variant fits together with.
    let mut fits = HashMap::new();
    for (self_id, self_variants) in pieces.iter() {
        for (self_variant_id, self_variant) in self_variants.iter().enumerate() {
            let mut above = HashSet::new();
            let mut left_of = HashSet::new();
            let mut right_of = HashSet::new();
            let mut below = HashSet::new();
            for (other_id, other_variants) in pieces.iter() {
                if self_id == other_id {
                    continue;
                }
                for (other_variant_id, other_variant) in other_variants.iter().enumerate() {
                    let (fits_above, fits_left, fits_right, fits_below) =
                        self_variant.fits_with(other_variant);
                    if fits_above {
                        above.insert((*other_id, other_variant_id));
                    }
                    if fits_left {
                        left_of.insert((*other_id, other_variant_id));
                    }
                    if fits_right {
                        right_of.insert((*other_id, other_variant_id));
                    }
                    if fits_below {
                        below.insert((*other_id, other_variant_id));
                    }
                }
            }
            fits.insert(
                (*self_id, self_variant_id),
                (above, left_of, right_of, below),
            );
        }
    }

    // Seed the queue for the search algorithm with all piece variants that can start in the top
    // corner. Since we know that edge pieces actually don't fit with other pieces we can prune a
    // lot of possibilities.
    let mut queue = Vec::new();
    for ((id, variant), (above, left_of, _, _)) in fits.iter() {
        if !above.is_empty() || !left_of.is_empty() {
            continue;
        }

        let mut used_pieces = HashMap::new();
        used_pieces.insert(*id, *variant);

        let mut locations = Vec::new();
        locations.push(*id);

        queue.push((used_pieces, locations));
    }

    // Search for possible tile configurations depth first by appending tiles to the right. If the
    // current tile it on the right hand side we check the tile below the start of the current row.
    let mut part_a = None;
    while !queue.is_empty() {
        let (used_pieces, locations) = queue.pop().unwrap();

        // Check if we have managed to place all tiles
        if locations.len() == pieces.len() {
            part_a = Some(
                locations[0]
                    * locations[side - 1]
                    * locations[locations.len() - side]
                    * locations[locations.len() - 1],
            );
            break;
        }

        // Find current column and row
        let x = (locations.len() - 1) % side;
        let y = (locations.len() - 1) / side;

        // Get set of possible pieces that can go in the next location
        let alts = if x == side - 1 {
            let above_id = locations[(y * side)];
            &fits[&(above_id, used_pieces[&above_id])].3
        } else {
            let left_of_id = *locations.last().unwrap();
            &fits[&(left_of_id, used_pieces[&left_of_id])].2
        };

        // Add new possible pieces to the queue stack
        for (other_id, other_variant_id) in alts {
            if used_pieces.contains_key(&other_id) {
                continue;
            }

            let mut used = used_pieces.clone();
            used.insert(*other_id, *other_variant_id);

            let mut loc = locations.clone();
            loc.push(*other_id);

            queue.push((used, loc));
        }
    }

    Ok((
        part_a.ok_or_else(|| anyhow!("No valid configuration of tiles found"))?,
        None,
    ))
}
