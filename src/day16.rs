use anyhow::{anyhow, Result};
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::ops::RangeInclusive;
use std::path::Path;

/// Return the set of valid rules for this value
fn valid_rules(
    rules: &HashMap<String, (RangeInclusive<usize>, RangeInclusive<usize>)>,
    value: usize,
) -> HashSet<&str> {
    let mut valid_fields = HashSet::new();
    for (field, (r1, r2)) in rules.iter() {
        if r1.contains(&value) || r2.contains(&value) {
            valid_fields.insert(field.as_str());
        }
    }
    valid_fields
}

fn parse_ticket(line: &str) -> Result<Vec<usize>> {
    line.split(",")
        .map(|n| -> Result<_> { Ok(n.parse()?) })
        .collect()
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    // Read input and split into segments (rules, my ticket and nearby_tickets)
    let input = std::fs::read_to_string(path)?;
    let mut sections = input.trim_end().split("\n\n");
    let rules_str = sections.next().unwrap();
    let my_ticket_str = sections.next().unwrap();
    let nearby_tickets_str = sections.next().unwrap();

    let my_ticket = parse_ticket(my_ticket_str.lines().last().unwrap())?;
    let nearby_tickets = nearby_tickets_str
        .lines()
        .skip(1)
        .map(parse_ticket)
        .collect::<Result<Vec<_>>>()?;

    let mut rules = HashMap::new();
    let rule_re = Regex::new(r"^([^:]+): (\d+)-(\d+) or (\d+)-(\d+)$")?;
    for rule_str in rules_str.lines() {
        let c = rule_re
            .captures(rule_str)
            .ok_or(anyhow!("Invalid rule {:?}", rule_str))?;
        rules.insert(
            c[1].to_owned(),
            (c[2].parse()?..=c[3].parse()?, c[4].parse()?..=c[5].parse()?),
        );
    }

    // Track each position with a set containing all possible rules that can govern it
    let all_fields = rules
        .keys()
        .map(|field| field.as_str())
        .collect::<HashSet<_>>();
    let mut possible_rules_by_slot = std::iter::repeat(all_fields.clone())
        .take(all_fields.len())
        .collect::<Vec<_>>();

    // Count all invalid tickets. If the tickets are valid, use that to reduce the set
    // of possible rules for that position
    let mut num_invalid_tickets = 0;
    for ticket in nearby_tickets {
        for (i, field) in ticket.into_iter().enumerate() {
            let possible_rules = valid_rules(&rules, field);
            if possible_rules.len() == 0 {
                num_invalid_tickets += field;
            } else {
                possible_rules_by_slot[i].retain(|f| possible_rules.contains(f));
            }
        }
    }

    // When reduction is done we don't actually uniquely know each which rule that governs
    // each slot. So we iteratively reduce the set of possible rules for each slot by
    // putting all slots with a single rule in a separate set and then eliminate them from
    // all other slots. This process of elimiation continues until all rules have been
    // reduced to a single possible slot.
    //
    // Note that this could get stuck in an infinite loop if there is no solution
    let mut singles = HashSet::new();
    loop {
        let mut done = true;
        for slot in possible_rules_by_slot.iter_mut() {
            if slot.len() == 1 {
                singles.insert(slot.iter().next().unwrap().to_owned());
            } else {
                for single in singles.iter() {
                    slot.remove(single);
                }
                slot.retain(|f| !singles.contains(f));
                done = false;
            }
        }

        if done {
            break;
        }
    }

    // Translate the nested slot -> set structure to a list of rule names
    let rules_by_slot = possible_rules_by_slot
        .into_iter()
        .map(|f| f.into_iter().next().unwrap())
        .collect::<Vec<_>>();

    let part_b = my_ticket
        .into_iter()
        .zip(rules_by_slot.into_iter())
        .filter_map(|(value, rule)| {
            if rule.starts_with("departure") {
                Some(value)
            } else {
                None
            }
        })
        .fold(1, |product, value| product * value);

    Ok((num_invalid_tickets, Some(part_b)))
}
