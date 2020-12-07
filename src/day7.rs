use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::path::Path;

use crate::reader::read_lines;

/// Split the string at the given separator. If the separator is not found, the
/// second part of the tuple will be None.
fn split_once<'a>(s: &'a str, pat: &str) -> (&'a str, Option<&'a str>) {
    let del_len = pat.len();
    match s.find(pat) {
        Some(i) => (&s[..i], Some(&s[i + del_len..])),
        None => (s, None),
    }
}

fn parse_bag_color_with_count(bag_str: &str) -> Result<(usize, &str)> {
    let (n_str, color_str) = split_once(bag_str, " ");
    let color_str = color_str.ok_or(anyhow!("Invalid bag specification"))?;
    let bag_idx = color_str
        .rfind(" bag")
        .ok_or(anyhow!("Bag specification must end with bag(s)"))?;
    Ok((n_str.parse()?, &color_str[..bag_idx]))
}

fn can_contain(bags: &HashMap<String, HashMap<String, usize>>, parent: &str, child: &str) -> bool {
    if bags[parent].contains_key(child) {
        return true;
    } else {
        for c in bags[parent].keys() {
            if can_contain(bags, c, child) {
                return true;
            }
        }
    }
    false
}

fn num_bags_inside(bags: &HashMap<String, HashMap<String, usize>>, parent: &str) -> usize {
    let mut n = bags[parent].values().sum();
    for (k, v) in bags[parent].iter() {
        n += v * num_bags_inside(bags, k);
    }
    n
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let mut bags = HashMap::new();
    for line in read_lines(path)? {
        let line = line?;
        let (bag, content) = split_once(&line[..line.len() - 1], " bags contain ");

        bags.insert(
            bag.to_owned(),
            match content {
                Some("no other bags") => HashMap::new(),
                Some(bags_str) => {
                    let mut inner_bags = HashMap::new();
                    for sub_bag in bags_str.split(", ") {
                        let (n, color) = parse_bag_color_with_count(sub_bag)?;
                        inner_bags.insert(color.to_owned(), n);
                    }
                    inner_bags
                }
                None => {
                    return Err(anyhow!("No"));
                }
            },
        );
    }
    Ok((
        bags.keys()
            .filter(|x| can_contain(&bags, x, "shiny gold"))
            .count(),
        Some(num_bags_inside(&bags, "shiny gold")),
    ))
}
