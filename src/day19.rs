use anyhow::{anyhow, Result};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::str::FromStr;

use crate::reader::split_once;

#[derive(Debug)]
enum Rule {
    Literal(String),
    Meta(Vec<Vec<usize>>),
}

impl Rule {
    fn resolve(&self, rules: &HashMap<usize, Rule>) -> Vec<String> {
        let mut out = Vec::new();
        match self {
            Self::Literal(s) => {
                out.push(s.to_owned());
            }
            Self::Meta(r) => {
                for sub_rule in r {
                    let mut parts = Vec::new();
                    for rule_no in sub_rule {
                        parts.push(rules.get(rule_no).unwrap().resolve(rules));
                    }
                    for x in parts.into_iter().multi_cartesian_product() {
                        let mut a = String::new();
                        a.extend(x);
                        out.push(a);
                    }
                }
            }
        }
        out
    }
}

impl FromStr for Rule {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        if s.len() == 3 && s.starts_with('"') && s.ends_with('"') {
            Ok(Self::Literal(s[1..2].to_owned()))
        } else {
            Ok(Self::Meta(
                s.split(" | ")
                    .map(|group| {
                        Ok(group
                            .split(' ')
                            .map(usize::from_str)
                            .collect::<Result<Vec<usize>, _>>()?)
                    })
                    .collect::<Result<Vec<_>>>()?,
            ))
        }
    }
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let input_str = std::fs::read_to_string(path)?;
    let (rules_str, data_str) = split_once(&input_str, "\n\n");

    let rules = rules_str
        .lines()
        .map(|l| {
            if let (n, Some(rule_str)) = split_once(l, ": ") {
                Ok((n.parse::<usize>()?, Rule::from_str(rule_str)?))
            } else {
                Err(anyhow!("Invalid rule {:?}", l))
            }
        })
        .collect::<Result<HashMap<_, _>>>()?;

    // Get the complete list of valid messages according to these three rules
    let rule_0 = rules[&0]
        .resolve(&rules)
        .into_iter()
        .collect::<HashSet<String>>();
    let rule_42 = rules[&42]
        .resolve(&rules)
        .into_iter()
        .collect::<HashSet<String>>();
    let rule_31 = rules[&31]
        .resolve(&rules)
        .into_iter()
        .collect::<HashSet<String>>();

    // Split the messages into two sets. The first contains all messages that are valid in part A.
    // These messages are still valid in part B.
    let (part_a_msgs, rest): (Vec<_>, Vec<_>) = data_str
        .ok_or_else(|| anyhow!("No messages found in input"))?
        .lines()
        .into_iter()
        .partition(|m| rule_0.contains(m.to_owned()));

    // For part B we have the following three rules:
    //
    //  0: 8 11
    //  8: 42 | 42 8  (42 is repeated one or more times)
    // 11: 42 31 | 42 11 31  (first we have at least one 42, then we have an equal number of 31)
    //
    // Putting this into words mean that:
    //
    // - The message must start with a chunk matching 42
    // - The message must end with a chunk matching 31
    // - All chunks matching 42 appears before any match of 31
    // - There must be `n + 1` chunks matching 42 and `n` chunks matching 31
    let mut part_b = 0;
    for msg in rest {
        // NOTE: This solution is only valid if there is no overlap between rule 42 and 31
        let len = msg.len();
        if len < 24 || len % 8 != 0 {
            continue;
        }

        if !rule_42.contains(&msg[0..8]) {
            continue;
        }

        if !rule_31.contains(&msg[len - 8..]) {
            continue;
        }

        let num_chunks = len / 8;

        let mut num_31 = 1;
        while num_31 < num_chunks / 2 {
            let chunk_start = len - 8 - num_31 * 8;
            if !rule_31.contains(&msg[chunk_start..chunk_start + 8]) {
                break;
            }
            num_31 += 1;
        }

        let mut num_42 = 1;
        while num_31 + num_42 < num_chunks {
            let chunk_start = num_42 * 8;
            if !rule_42.contains(&msg[chunk_start..chunk_start + 8]) {
                break;
            }
            num_42 += 1;
        }

        if num_31 + num_42 == num_chunks && num_42 > num_31 {
            part_b += 1;
        }
    }

    Ok((part_a_msgs.len(), Some(part_a_msgs.len() + part_b)))
}
