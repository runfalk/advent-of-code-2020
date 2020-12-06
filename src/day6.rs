use anyhow::Result;
use std::collections::HashMap;
use std::path::Path;

use crate::reader::read_lines;

#[derive(Debug, Default, PartialEq, Eq)]
struct Group {
    num_people: usize,
    answers: HashMap<char, usize>,
}

impl Group {
    fn new() -> Self {
        Default::default()
    }

    fn add_person(&mut self, answers: impl Iterator<Item = char>) {
        self.num_people += 1;
        for c in answers {
            *self.answers.entry(c).or_insert(0) += 1;
        }
    }

    fn num_unique_yes(&self) -> usize {
        self.answers.len()
    }

    fn num_unanimous_yes(&self) -> usize {
        self.answers
            .values()
            .filter(|n| **n == self.num_people)
            .count()
    }
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let mut groups = Vec::new();
    groups.push(Group::new());
    for line in read_lines(path)? {
        let line = line?;
        if line == "" {
            groups.push(Group::new());
            continue;
        }
        groups.last_mut().unwrap().add_person(line.chars());
    }
    Ok((
        groups.iter().map(|g| g.num_unique_yes()).sum(),
        Some(groups.iter().map(|g| g.num_unanimous_yes()).sum()),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group() {
        let mut group = Group::new();
        group.add_person("ab".chars());
        group.add_person("ac".chars());
        assert_eq!(group.num_people, 2);
        assert_eq!(group.num_unique_yes(), 3);
        assert_eq!(group.num_unanimous_yes(), 1);

        let mut group = Group::new();
        group.add_person("a".chars());
        group.add_person("b".chars());
        group.add_person("c".chars());
        assert_eq!(group.num_people, 3);
        assert_eq!(group.num_unique_yes(), 3);
        assert_eq!(group.num_unanimous_yes(), 0);
    }
}
