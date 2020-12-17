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
    let color_str = color_str.ok_or_else(|| anyhow!("Invalid bag specification"))?;
    let bag_idx = color_str
        .rfind(" bag")
        .ok_or_else(|| anyhow!("Bag specification must end with bag(s)"))?;
    Ok((n_str.parse()?, &color_str[..bag_idx]))
}

fn can_contain(bags: &HashMap<String, HashMap<String, usize>>, parent: &str, child: &str) -> bool {
    let bag = &bags[parent];
    for inner_bag in bag.keys() {
        if inner_bag == child || can_contain(bags, inner_bag, child) {
            return true;
        }
    }
    false
}

fn num_bags_inside(bags: &HashMap<String, HashMap<String, usize>>, parent: &str) -> usize {
    let mut n = bags[parent].values().sum();
    for (inner_bag, count) in bags[parent].iter() {
        n += count * num_bags_inside(bags, inner_bag);
    }
    n
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let mut bags = HashMap::new();
    for line in read_lines(path)? {
        let line = line?;
        let (bag, content) = split_once(&line[..line.len() - 1], " bags contain ");
        let inner_bags = match content {
            Some("no other bags") => HashMap::new(),
            Some(bags_str) => {
                let mut inner_bags = HashMap::new();
                for inner_bag_str in bags_str.split(", ") {
                    let (n, color) = parse_bag_color_with_count(inner_bag_str)?;
                    inner_bags.insert(color.to_owned(), n);
                }
                inner_bags
            }
            None => {
                return Err(anyhow!("Invalid line, must contain ' bags contain '"));
            }
        };
        bags.insert(bag.to_owned(), inner_bags);
    }
    Ok((
        bags.keys()
            .filter(|bag| can_contain(&bags, bag, "shiny gold"))
            .count(),
        Some(num_bags_inside(&bags, "shiny gold")),
    ))
}
