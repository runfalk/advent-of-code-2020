use anyhow::{anyhow, Result};
use std::path::Path;
use std::str::FromStr;

use crate::coord::{Coord, Direction};
use crate::reader::read_parsed_lines;

#[derive(Debug)]
enum Action {
    North(usize),
    South(usize),
    East(usize),
    West(usize),
    Left(usize),
    Right(usize),
    Forward(usize),
}

fn num_turns(degrees: usize) -> Result<usize> {
    match degrees {
        90 => Ok(1),
        180 => Ok(2),
        270 => Ok(3),
        _ => Err(anyhow!(
            "Degrees must be either 90, 180 or 270, got {}",
            degrees
        )),
    }
}

impl FromStr for Action {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        Ok(
            match s.chars().next().ok_or(anyhow!("Invalid line, empty"))? {
                'N' => Action::North(s[1..].parse()?),
                'S' => Action::South(s[1..].parse()?),
                'E' => Action::East(s[1..].parse()?),
                'W' => Action::West(s[1..].parse()?),
                'L' => Action::Left(num_turns(s[1..].parse()?)?),
                'R' => Action::Right(num_turns(s[1..].parse()?)?),
                'F' => Action::Forward(s[1..].parse()?),
                c => return Err(anyhow!("Got invalid action ({})", c)),
            },
        )
    }
}

fn part_a(actions: &[Action]) -> Result<usize> {
    let mut ship = Coord::origin();
    let mut dir = Direction::Right(1);
    for action in actions {
        match action {
            Action::North(n) => {
                ship.offset_mut(Direction::Up(*n));
            }
            Action::South(n) => {
                ship.offset_mut(Direction::Down(*n));
            }
            Action::East(n) => {
                ship.offset_mut(Direction::Right(*n));
            }
            Action::West(n) => {
                ship.offset_mut(Direction::Left(*n));
            }
            Action::Left(n) => {
                for _ in 0..*n {
                    dir = dir.turn_left();
                }
            }
            Action::Right(n) => {
                for _ in 0..*n {
                    dir = dir.turn_right();
                }
            }
            Action::Forward(n) => {
                ship.offset_mut(dir.resize(*n));
            }
        }
    }
    Ok(ship.x.abs() as usize + ship.y.abs() as usize)
}

fn part_b(actions: &[Action]) -> Result<usize> {
    let mut ship = Coord::origin();
    let mut waypoint = Coord::new(10, -1);
    for action in actions {
        match action {
            Action::North(n) => {
                waypoint.offset_mut(Direction::Up(*n));
            }
            Action::South(n) => {
                waypoint.offset_mut(Direction::Down(*n));
            }
            Action::East(n) => {
                waypoint.offset_mut(Direction::Right(*n));
            }
            Action::West(n) => {
                waypoint.offset_mut(Direction::Left(*n));
            }
            Action::Left(n) => {
                for _ in 0..*n {
                    waypoint = Coord::new(waypoint.y, -waypoint.x);
                }
            }
            Action::Right(n) => {
                for _ in 0..*n {
                    waypoint = Coord::new(-waypoint.y, waypoint.x);
                }
            }
            Action::Forward(n) => {
                for _ in 0..*n {
                    ship += waypoint;
                }
            }
        }
    }
    Ok(ship.x.abs() as usize + ship.y.abs() as usize)
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let actions = read_parsed_lines(path)?.collect::<Result<Vec<Action>>>()?;
    Ok((part_a(&actions)?, Some(part_b(&actions)?)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> Result<()> {
        let actions = vec!["F10", "N3", "F7", "R90", "F11"]
            .into_iter()
            .map(str::parse)
            .collect::<Result<Vec<Action>>>()?;
        assert_eq!(part_a(&actions)?, 25);
        assert_eq!(part_b(&actions)?, 286);
        Ok(())
    }
}
