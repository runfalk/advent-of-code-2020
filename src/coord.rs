use std::ops::{Add, Sub};

use self::Direction::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Coord {
    pub x: isize,
    pub y: isize,
}

/// Coordinate system assumes down is positive and up is negative
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Direction {
    Up(usize),
    Right(usize),
    Down(usize),
    Left(usize),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoordNeighbors {
    origin: Coord,
    i: usize,
    step: usize,
}

impl Coord {
    pub fn new(x: isize, y: isize) -> Self {
        Coord { x, y }
    }

    pub fn origin() -> Self {
        Coord { x: 0, y: 0 }
    }

    pub fn distance(a: Self, b: Self) -> usize {
        let relative_coord = a - b;
        (relative_coord.x.abs() + relative_coord.y.abs()) as usize
    }

    pub fn distance_from_origin(&self) -> usize {
        Self::distance(*self, Self::origin())
    }

    pub fn iter_neighbors(&self) -> CoordNeighbors {
        CoordNeighbors::new(*self)
    }

    pub fn iter_all_neighbors(&self) -> CoordNeighbors {
        CoordNeighbors::new_with_diagonals(*self)
    }

    pub fn up(&self) -> Self {
        self.offset(Up(1))
    }

    pub fn offset(&self, dir: Direction) -> Self {
        match dir {
            Up(i) => Coord::new(self.x, self.y - i as isize),
            Right(i) => Coord::new(self.x + i as isize, self.y),
            Down(i) => Coord::new(self.x, self.y + i as isize),
            Left(i) => Coord::new(self.x - i as isize, self.y),
        }
    }
}

macro_rules! coord_op_overload {
    ($a:ty, $b:ty) => {
        impl Add<$b> for $a {
            type Output = Coord;
            fn add(self, rhs: $b) -> Self::Output {
                Coord {
                    x: self.x + rhs.x,
                    y: self.y + rhs.y,
                }
            }
        }

        impl Sub<$b> for $a {
            type Output = Coord;
            fn sub(self, rhs: $b) -> Self::Output {
                Coord {
                    x: self.x - rhs.x,
                    y: self.y - rhs.y,
                }
            }
        }
    };
}

coord_op_overload!(Coord, Coord);
coord_op_overload!(Coord, &Coord);
coord_op_overload!(&Coord, Coord);
coord_op_overload!(&Coord, &Coord);

impl Direction {
    pub fn resize(&self, len: usize) -> Direction {
        match self {
            Up(_) => Up(len),
            Right(_) => Right(len),
            Down(_) => Down(len),
            Left(_) => Left(len),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Up(n) => *n,
            Right(n) => *n,
            Down(n) => *n,
            Left(n) => *n,
        }
    }
}

impl CoordNeighbors {
    pub fn new(origin: Coord) -> Self {
        Self {
            origin,
            i: 0,
            step: 2,
        }
    }

    pub fn new_with_diagonals(origin: Coord) -> Self {
        Self {
            origin,
            i: 0,
            step: 1,
        }
    }
}

impl Iterator for CoordNeighbors {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        let output = match self.i {
            0 => Some(self.origin.offset(Up(1))),
            1 => Some(self.origin.offset(Up(1)).offset(Right(1))),
            2 => Some(self.origin.offset(Right(1))),
            3 => Some(self.origin.offset(Right(1)).offset(Down(1))),
            4 => Some(self.origin.offset(Down(1))),
            5 => Some(self.origin.offset(Down(1)).offset(Left(1))),
            6 => Some(self.origin.offset(Left(1))),
            7 => Some(self.origin.offset(Left(1)).offset(Up(1))),
            _ => None,
        };
        self.i += self.step;

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(Coord::new(1, 3) + Coord::new(2, 4), Coord::new(3, 7));
    }

    #[test]
    fn test_sub() {
        assert_eq!(Coord::new(1, 3) - Coord::new(2, 4), Coord::new(-1, -1));
    }

    #[test]
    fn test_distance() {
        let a = Coord::new(3, 4);
        let b = Coord::new(1, 1);

        assert_eq!(a.distance_from_origin(), 7);
        assert_eq!(b.distance_from_origin(), 2);

        assert_eq!(Coord::distance(a, b), 5);
    }

    #[test]
    fn test_offset() {
        assert_eq!(Coord::origin().offset(Up(100)), Coord::new(0, -100));
        assert_eq!(Coord::origin().offset(Right(100)), Coord::new(100, 0));
        assert_eq!(Coord::origin().offset(Down(100)), Coord::new(0, 100));
        assert_eq!(Coord::origin().offset(Left(100)), Coord::new(-100, 0));
    }

    #[test]
    fn test_neighbors() {
        assert_eq!(
            Coord::new(10, 10).iter_neighbors().collect::<Vec<_>>(),
            vec![
                Coord::new(10, 9),
                Coord::new(11, 10),
                Coord::new(10, 11),
                Coord::new(9, 10),
            ],
        );
    }
}
