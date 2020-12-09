use anyhow::{anyhow, Result};
use std::collections::HashSet;
use std::path::Path;
use std::str::FromStr;

use crate::reader::read_parsed_lines;

#[derive(Debug, Clone)]
enum Instruction {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}

#[derive(Debug)]
enum State {
    Halted(isize),
    Repeated(isize),
    Error(anyhow::Error),
}

impl Instruction {
    fn flip(&mut self) {
        *self = match self {
            Instruction::Acc(i) => Instruction::Acc(*i),
            Instruction::Jmp(i) => Instruction::Nop(*i),
            Instruction::Nop(i) => Instruction::Jmp(*i),
        };
    }
}

impl FromStr for Instruction {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match split_once(s, " ") {
            ("acc", Some(i_str)) => Ok(Instruction::Acc(i_str.parse()?)),
            ("jmp", Some(i_str)) => Ok(Instruction::Jmp(i_str.parse()?)),
            ("nop", Some(i_str)) => Ok(Instruction::Nop(i_str.parse()?)),
            _ => Err(anyhow!("Invalid instruction {:?}", s)),
        }
    }
}

fn split_once<'a>(s: &'a str, pat: &str) -> (&'a str, Option<&'a str>) {
    let del_len = pat.len();
    match s.find(pat) {
        Some(i) => (&s[..i], Some(&s[i + del_len..])),
        None => (s, None),
    }
}

fn run(program: &[Instruction]) -> State {
    let mut prev_instrs = HashSet::new();
    let mut ptr = 0isize;
    let mut reg = 0isize;
    while ptr != program.len() as isize {
        // Check if we have visited this instruction previously
        if prev_instrs.contains(&ptr) {
            return State::Repeated(reg);
        }
        prev_instrs.insert(ptr);

        match program.get(ptr as usize) {
            Some(Instruction::Nop(_)) => {
                ptr += 1;
            }
            Some(Instruction::Acc(i)) => {
                reg += i;
                ptr += 1;
            }
            Some(Instruction::Jmp(i)) => {
                ptr += i;
            }
            None => {
                return State::Error(anyhow!("Current instruction ({}) is out of bounds", ptr));
            }
        };
    }
    State::Halted(reg)
}

pub fn main(path: &Path) -> Result<(isize, Option<isize>)> {
    let mut program = read_parsed_lines(path)?.collect::<Result<Vec<Instruction>>>()?;
    let part_a = if let State::Repeated(reg) = run(&program) {
        reg
    } else {
        return Err(anyhow!("Part A is expected to run in an infinite loop"));
    };

    let mut part_b = None;
    for i in 0..program.len() {
        program[i].flip();
        if let State::Halted(reg) = run(&program) {
            part_b = Some(reg);
            break;
        }
        program[i].flip();
    }

    Ok((part_a, part_b))
}
