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

    /// Returns a tuple that shows where this piece fits with the other one
    fn fits_with(&self, other: &Self) -> (bool, bool, bool, bool) {
        let top = self.top == other.bottom;
        let left = self.left == other.right;
        let right = self.right == other.left;
        let bottom = self.bottom == other.top;
        (top, left, right, bottom)
    }
}

fn all_matrix_transforms(matrix: &[Vec<bool>]) -> Vec<Vec<Vec<bool>>> {
    let mut matrices = Vec::new();

    for i in 0..8 {
        let mut matrix = matrix.to_owned();

        let transpose = (i >> 2) == 1;
        let flip_x = (i >> 1) & 1 == 1;
        let flip_y = i & 1 == 1;

        if transpose {
            let mut new_matrix = Vec::new();
            for y in 0..matrix[0].len() {
                let mut row = Vec::new();
                #[allow(clippy::needless_range_loop)]
                for x in 0..matrix.len() {
                    row.push(matrix[x][y]);
                }
                new_matrix.push(row);
            }
            matrix = new_matrix;
        }

        if flip_x {
            for row in matrix.iter_mut() {
                row.reverse();
            }
        }

        if flip_y {
            matrix.reverse();
        }

        matrices.push(matrix);
    }
    matrices
}

fn num_non_sea_monster_pixels(image: &[bool], width: usize, height: usize) -> usize {
    // The sea monster pattern looks like this. We translate it to the correct offsets
    //                   #
    // #    ##    ##    ###
    //  #  #  #  #  #  #
    let sea_monster_pattern = &[
        18,
        width,
        width + 5,
        width + 6,
        width + 11,
        width + 12,
        width + 17,
        width + 18,
        width + 19,
        2 * width + 1,
        2 * width + 4,
        2 * width + 7,
        2 * width + 10,
        2 * width + 13,
        2 * width + 16,
    ];
    let mut sea_monster_offsets = HashSet::new();
    for y in 0..height - 2 {
        for x in 0..width - 19 {
            let offset_base = y * width + x;
            let aligned_offsets = sea_monster_pattern
                .iter()
                .map(|offset| offset_base + offset);
            if aligned_offsets.clone().all(|offset| image[offset]) {
                sea_monster_offsets.extend(aligned_offsets);
            }
        }
    }
    image.iter().filter(|&v| *v).count() - sea_monster_offsets.len()
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
            Ok((id, all_matrix_transforms(&matrix)))
        })
        .collect::<Result<HashMap<_, _>>>()?;

    // Generate a mapping from tile ID to a list of all variations of that piece. We only store the
    // edges and provide some convenience methods to check if two variations fit together.
    let pieces = piece_matrices
        .iter()
        .map(|(id, matrices)| {
            (
                *id,
                matrices
                    .iter()
                    .map(|matrix| Piece::from_matrix(matrix))
                    .collect::<Vec<_>>(),
            )
        })
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
    let mut tile_configuration = None;
    while !queue.is_empty() {
        let (used_pieces, locations) = queue.pop().unwrap();

        // Check if we have managed to place all tiles
        if locations.len() == pieces.len() {
            let tiles = locations
                .into_iter()
                .map(|id| (id, used_pieces[&id]))
                .collect::<Vec<_>>();
            tile_configuration = Some(tiles);
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

    // Use the configuration of tiles to find the product of the corner tile IDs
    let tile_configuration =
        tile_configuration.ok_or_else(|| anyhow!("No valid configuration of tiles found"))?;
    let part_a = tile_configuration[0].0
        * tile_configuration[side - 1].0
        * tile_configuration[tile_configuration.len() - side].0
        * tile_configuration[tile_configuration.len() - 1].0;

    // Allocate all rows in the matrix that holds the final image with all borders between tiles
    // removed
    let tile_side = piece_matrices.values().next().unwrap()[0].len() - 2;
    let mut full_image = Vec::new();
    for _ in 0..side * tile_side {
        full_image.push(Vec::with_capacity(side * tile_side));
    }

    // Fill final image with pixel values
    for (i, (id, variant)) in tile_configuration.into_iter().enumerate() {
        let tile_y = i / side;
        for (j, row) in piece_matrices[&id][variant]
            .iter()
            .skip(1)
            .take(tile_side)
            .enumerate()
        {
            full_image[tile_y * tile_side + j].extend(row.iter().skip(1).take(tile_side).copied());
        }
    }

    // Try all different transformations of final image, since it may be flipped incorrectly for
    // detecting sea monsters
    let mut part_b = usize::MAX;
    for image_variant in all_matrix_transforms(&full_image) {
        let width = side * tile_side;
        let pixels = image_variant
            .into_iter()
            .map(|line| line.into_iter())
            .flatten()
            .collect::<Vec<_>>();
        part_b = part_b.min(num_non_sea_monster_pixels(&pixels, width, width));
    }

    Ok((part_a, Some(part_b)))
}
