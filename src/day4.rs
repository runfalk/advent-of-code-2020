use anyhow::{anyhow, Error, Result};
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::str::FromStr;

use crate::reader::read_lines;

static HEIGHT_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(\d+)(cm|in)$").unwrap());
static COLOR_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^#[a-f0-9]{6}$").unwrap());
static EYE_COLOR_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap());
static PID_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\d{9}$").unwrap());

enum Height {
    Cm(usize),
    In(usize),
}

impl FromStr for Height {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let c = HEIGHT_RE.captures(s).ok_or(anyhow!("Invalid height"))?;
        let num = c[1].parse()?;
        Ok(match &c[2] {
            "cm" => Self::Cm(num),
            "in" => Self::In(num),
            _ => unreachable!(),
        })
    }
}

pub fn part2(passport: &HashMap<String, String>) -> Result<()> {
    let byr: usize = passport
        .get("byr")
        .ok_or(anyhow!("No byr field"))?
        .parse()?;
    if !(1920..=2002).contains(&byr) {
        return Err(anyhow!("Invalid byr"));
    }

    let iyr: usize = passport
        .get("iyr")
        .ok_or(anyhow!("No iyr field"))?
        .parse()?;
    if !(2010..=2020).contains(&iyr) {
        return Err(anyhow!("Invalid iyr"));
    }

    let eyr: usize = passport
        .get("eyr")
        .ok_or(anyhow!("No eyr field"))?
        .parse()?;
    if !(2020..=2030).contains(&eyr) {
        return Err(anyhow!("Invalid eyr"));
    }

    match passport
        .get("hgt")
        .ok_or(anyhow!("No hgt field"))?
        .parse()?
    {
        Height::Cm(150..=193) => (),
        Height::In(59..=76) => (),
        _ => return Err(anyhow!("Invalid hgt")),
    };

    if !COLOR_RE.is_match(passport.get("hcl").ok_or(anyhow!("No hcl field"))?) {
        return Err(anyhow!("Invalid hcl"));
    }

    if !EYE_COLOR_RE.is_match(passport.get("ecl").ok_or(anyhow!("No ecl field"))?) {
        return Err(anyhow!("Invalid ecl"));
    }

    if !PID_RE.is_match(passport.get("pid").ok_or(anyhow!("No pid field"))?) {
        return Err(anyhow!("Invalid pid"));
    }

    Ok(())
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let re = Regex::new(r"([^: ]+):(\S+)")?;
    let mut passports = Vec::new();
    passports.push(HashMap::new());
    for line in read_lines(path)? {
        let line = line?;
        if line == "" {
            passports.push(HashMap::new());
        }
        for c in re.captures_iter(&line) {
            passports
                .last_mut()
                .unwrap()
                .insert(c[1].to_owned(), c[2].to_owned());
        }
    }
    passports.push(HashMap::new());

    let mut num_valid_a = 0;
    let mut num_valid_b = 0;
    let required_fields: HashSet<_> = vec![
        "byr".to_owned(),
        "iyr".to_owned(),
        "eyr".to_owned(),
        "hgt".to_owned(),
        "hcl".to_owned(),
        "ecl".to_owned(),
        "pid".to_owned(),
    ]
    .into_iter()
    .collect();
    for p in passports {
        let fields: HashSet<_> = p.keys().cloned().collect();
        if required_fields.difference(&fields).count() == 0 {
            num_valid_a += 1;
            if part2(&p).is_ok() {
                num_valid_b += 1;
            }
        }
    }

    Ok((num_valid_a, Some(num_valid_b)))
}
