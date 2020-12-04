use anyhow::{anyhow, Error, Result};
use once_cell::sync::Lazy;
use regex::Regex;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;
use std::path::Path;
use std::str::FromStr;

use crate::reader::read_lines;

#[derive(Debug)]
struct KeyError<'a, T: ?Sized> {
    key: &'a T,
}

impl<'a, T: fmt::Display + ?Sized> fmt::Display for KeyError<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "No key {}", self.key)
    }
}

impl<'a, T: fmt::Debug + fmt::Display + ?Sized> std::error::Error for KeyError<'a, T> {}

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

fn get_field<'a, 'b, K, V, Q>(
    map: &'a HashMap<K, V>,
    key: &'b Q,
) -> std::result::Result<&'a V, KeyError<'b, Q>>
where
    K: Eq + Hash + Borrow<Q>,
    Q: Eq + Hash + fmt::Display + ?Sized,
{
    map.get(key).ok_or_else(|| KeyError { key })
}

pub fn validate_passport(passport: &HashMap<String, String>) -> Result<()> {
    // We need to extract the all fields before value validation since part A
    // only requires the field to be present
    let byr = get_field(passport, "byr")?;
    let iyr = get_field(passport, "iyr")?;
    let eyr = get_field(passport, "eyr")?;
    let hgt = get_field(passport, "hgt")?;
    let hcl = get_field(passport, "hcl")?;
    let ecl = get_field(passport, "ecl")?;
    let pid = get_field(passport, "pid")?;

    if !(1920..=2002).contains(&byr.parse::<usize>()?) {
        return Err(anyhow!("Invalid byr"));
    }

    if !(2010..=2020).contains(&iyr.parse::<usize>()?) {
        return Err(anyhow!("Invalid iyr"));
    }

    if !(2020..=2030).contains(&eyr.parse::<usize>()?) {
        return Err(anyhow!("Invalid eyr"));
    }

    match hgt.parse()? {
        Height::Cm(150..=193) => (),
        Height::In(59..=76) => (),
        _ => return Err(anyhow!("Invalid hgt")),
    };

    if !COLOR_RE.is_match(hcl) {
        return Err(anyhow!("Invalid hcl"));
    }

    if !EYE_COLOR_RE.is_match(ecl) {
        return Err(anyhow!("Invalid ecl"));
    }

    if !PID_RE.is_match(pid) {
        return Err(anyhow!("Invalid pid"));
    }

    Ok(())
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    // Read all passports in a vector of maps from field to value
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

    let mut num_valid_a = 0;
    let mut num_valid_b = 0;
    for p in passports {
        match validate_passport(&p) {
            Ok(_) => {
                num_valid_a += 1;
                num_valid_b += 1;
            }
            Err(e) if !e.is::<KeyError<'_, str>>() => {
                num_valid_a += 1;
            }
            Err(_) => {}
        }
    }

    Ok((num_valid_a, Some(num_valid_b)))
}
