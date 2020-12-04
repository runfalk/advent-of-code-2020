use anyhow::Result;
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;
use std::path::Path;
use thiserror::Error;

use crate::reader::read_lines;

#[derive(Error, Debug)]
enum PassportError {
    #[error("missing field {0:?}")]
    Missing(String),

    #[error("invalid value {1:?} for field {0:?}")]
    Invalid(String, String),
}

static HEIGHT_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(\d+)(cm|in)$").unwrap());
static COLOR_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^#[a-f0-9]{6}$").unwrap());
static EYE_COLOR_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap());
static PID_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\d{9}$").unwrap());

fn get_field<'a>(map: &'a HashMap<String, String>, key: &str) -> Result<&'a str, PassportError> {
    map.get(key)
        .ok_or_else(|| PassportError::Missing(key.to_owned()))
        .map(String::as_str)
}

fn validate_passport(passport: &HashMap<String, String>) -> Result<(), PassportError> {
    // We need to extract the all fields before value validation since part A
    // only requires the field to be present
    let byr = get_field(passport, "byr")?;
    let iyr = get_field(passport, "iyr")?;
    let eyr = get_field(passport, "eyr")?;
    let hgt = get_field(passport, "hgt")?;
    let hcl = get_field(passport, "hcl")?;
    let ecl = get_field(passport, "ecl")?;
    let pid = get_field(passport, "pid")?;

    if !(1920..=2002).contains(&byr.parse::<usize>().unwrap_or(0)) {
        return Err(PassportError::Invalid("byr".to_owned(), byr.into()));
    }

    if !(2010..=2020).contains(&iyr.parse::<usize>().unwrap_or(0)) {
        return Err(PassportError::Invalid("iyr".to_owned(), iyr.into()));
    }

    if !(2020..=2030).contains(&eyr.parse::<usize>().unwrap_or(0)) {
        return Err(PassportError::Invalid("eyr".to_owned(), eyr.into()));
    }

    HEIGHT_RE
        .captures(hgt)
        .map(|c| match &c[2] {
            "cm" if (150..=193).contains(&c[1].parse::<usize>().unwrap_or(0)) => Some(()),
            "in" if (59..=76).contains(&c[1].parse::<usize>().unwrap_or(0)) => Some(()),
            _ => None,
        })
        .flatten()
        .ok_or_else(|| PassportError::Invalid("hgt".to_owned(), hgt.into()))?;

    if !COLOR_RE.is_match(hcl) {
        return Err(PassportError::Invalid("hcl".to_owned(), hcl.into()));
    }

    if !EYE_COLOR_RE.is_match(ecl) {
        return Err(PassportError::Invalid("ecl".to_owned(), ecl.into()));
    }

    if !PID_RE.is_match(pid) {
        return Err(PassportError::Invalid("pid".to_owned(), pid.into()));
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
            Err(PassportError::Missing(_)) => {
                num_valid_a += 1;
            }
            Err(_) => {}
        }
    }

    Ok((num_valid_a, Some(num_valid_b)))
}
