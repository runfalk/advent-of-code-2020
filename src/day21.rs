use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::path::Path;

use crate::reader::{read_lines, split_once};

pub fn main(path: &Path) -> Result<(usize, Option<String>)> {
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

    // NOTE: This calculation will get stuck in an infinite loop if there are mutliple solutions,
    //       but the input is nice so we don't have to worry about that
    loop {
        let mut assigned_ingredients = HashSet::new();
        assigned_ingredients.extend(
            allergenes_to_ingredients
                .iter()
                .filter(|(_, v)| v.len() == 1)
                .map(|(_, v)| v.iter().next().unwrap().to_owned()),
        );

        if assigned_ingredients.len() == allergenes_to_ingredients.len() {
            break;
        }

        for (_, ingredients) in allergenes_to_ingredients.iter_mut() {
            if ingredients.len() == 1 {
                continue;
            }
            ingredients.retain(|ingredient| !assigned_ingredients.contains(ingredient));
        }
    }

    // Do some post processing to get the solution on the correct form
    let mut part_b_list = allergenes_to_ingredients
        .into_iter()
        .map(|(k, v)| (k, v.into_iter().next().unwrap()))
        .collect::<Vec<_>>();
    part_b_list.sort_unstable();
    let part_b = part_b_list
        .into_iter()
        .map(|(_, v)| v)
        .collect::<Vec<_>>()
        .join(",");

    Ok((part_a, Some(part_b)))
}
