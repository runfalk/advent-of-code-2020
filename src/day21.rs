use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::path::Path;

use crate::reader::{read_lines, split_once};

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let foods = read_lines(path)?
        .map(|l| -> Result<(HashSet<String>, HashSet<String>)> {
            let line = l?;
            let (ingredients_str, allergenes_str) = split_once(&line, " (contains ");

            let ingredients = ingredients_str.split(' ').map(|i| i.to_owned()).collect();
            let allergenes = if let Some(s) = allergenes_str {
                s[..s.len() - 1].split(", ").map(|a| a.to_owned()).collect()
            } else {
                HashSet::new()
            };

            Ok((ingredients, allergenes))
        })
        .collect::<Result<Vec<_>>>()?;

    let mut ingredient_count = HashMap::new();
    let mut allergenes_to_ingredients = HashMap::new();
    for (ingredients, allergenes) in foods.iter().cloned() {
        for allergene in allergenes {
            // Since only one ingredient can have each allergene the ingredient must appear every
            // time the allergene is listed
            let possible_ingredients = allergenes_to_ingredients
                .entry(allergene)
                .or_insert_with(|| ingredients.clone());
            possible_ingredients.retain(|ing| ingredients.contains(ing));
        }
        for ingredient in ingredients {
            *ingredient_count.entry(ingredient).or_insert(0) += 1;
        }
    }

    let ingredients_with_allergenes = allergenes_to_ingredients
        .values()
        .map(|i| i.iter())
        .flatten()
        .collect::<HashSet<_>>();

    let part_a = ingredient_count
        .into_iter()
        .filter(|(ingredient, _)| !ingredients_with_allergenes.contains(ingredient))
        .map(|(_, n)| n)
        .sum();
    Ok((part_a, None))
}
