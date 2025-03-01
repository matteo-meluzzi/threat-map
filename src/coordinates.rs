use core::ops::{Add, AddAssign, Neg, Sub};

use crate::{direction::Direction, orientation::Orientation, N};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub struct Coordinate {
    pub x: i8,
    pub y: i8,
}

impl Coordinate {
    pub fn new(x: i8, y: i8) -> Self {
        Coordinate { x, y }
    }

    pub fn from_index(index: usize) -> Option<Self> {        
        let center = N as i32 / 2;
        let row = index as i32 / N as i32;
        let col = index as i32 % N as i32;

        let x = col - center;
        let y = row - center;
        
        let bound = N as i32 / 2;
        if x >= -bound && y >= -bound && x <= bound && y <= bound {
            Some(Coordinate{ x: x as i8, y: y as i8 })
        } else {
            None
        }
    }

    pub fn to_index(&self) -> Option<usize> {
        let center: i32 = N as i32 / 2;

        let row = center + self.y as i32;
        let col = center + self.x as i32; 

        if row < N as i32 && col < N as i32 && row >= 0 && col >= 0 {
            Some(row as usize * N + col as usize)
        } else {
            None
        }

    }

    pub fn distance(&self, other: Coordinate) -> i32 {
        self.x.abs_diff(other.x) as i32 + self.y.abs_diff(other.y) as i32
    }

    fn rotate_right(&self) -> Coordinate {
        Coordinate { x: self.y, y: -self.x }
    }

    pub fn rotate(&self, direction: Direction) -> Coordinate {
        match direction {
            Direction::Front => *self,
            Direction::Right => self.rotate_right(),
            Direction::Back => self.rotate_right().rotate_right(),
            Direction::Left => self.rotate_right().rotate_right().rotate_right()
        }
    }

    fn in_direction_relative_to_north(&self, direction: Direction) -> Coordinate {
        match direction {
            Direction::Front => Coordinate { x: self.x, y: self.y - 1 },
            Direction::Right => Coordinate { x: self.x + 1, y: self.y },
            Direction::Back => Coordinate { x: self.x, y: self.y + 1 },
            Direction::Left => Coordinate { x: self.x - 1, y: self.y },
        }
    }

    pub fn in_direction(&self, direction: Direction, orientation: Orientation) -> Coordinate {        
        match (direction, orientation) {
            (Direction::Front, Orientation::North) => self.in_direction_relative_to_north(Direction::Front),
            (Direction::Right, Orientation::North) => self.in_direction_relative_to_north(Direction::Right),
            (Direction::Back, Orientation::North) => self.in_direction_relative_to_north(Direction::Back),
            (Direction::Left, Orientation::North) => self.in_direction_relative_to_north(Direction::Left),

            (Direction::Front, Orientation::East) => self.in_direction_relative_to_north(Direction::Right),
            (Direction::Right, Orientation::East) => self.in_direction_relative_to_north(Direction::Back),
            (Direction::Back, Orientation::East) => self.in_direction_relative_to_north(Direction::Left),
            (Direction::Left, Orientation::East) => self.in_direction_relative_to_north(Direction::Front),

            (Direction::Front, Orientation::South) => self.in_direction_relative_to_north(Direction::Back),
            (Direction::Right, Orientation::South) => self.in_direction_relative_to_north(Direction::Left),
            (Direction::Back, Orientation::South) => self.in_direction_relative_to_north(Direction::Front),
            (Direction::Left, Orientation::South) => self.in_direction_relative_to_north(Direction::Right),

            (Direction::Front, Orientation::West) => self.in_direction_relative_to_north(Direction::Left),
            (Direction::Right, Orientation::West) => self.in_direction_relative_to_north(Direction::Front),
            (Direction::Back, Orientation::West) => self.in_direction_relative_to_north(Direction::Right),
            (Direction::Left, Orientation::West) => self.in_direction_relative_to_north(Direction::Back),
        }
        
    }

    pub fn orientate_north(&self, orientation: Orientation) -> Coordinate {
        self.rotate(Orientation::North.direction_relative_to(orientation))
    }

    pub fn is_corner(&self) -> bool {
        (self.x == -(N as i8)/2 && self.y == -(N as i8)/2) ||
        (self.x == -(N as i8)/2 && self.y ==  (N as i8)/2) || 
        (self.x ==  (N as i8)/2 && self.y == -(N as i8)/2) || 
        (self.x ==  (N as i8)/2 && self.y ==  (N as i8)/2)
    }

    pub fn normalized(&self) -> Coordinate {
        if self.x.abs() > self.y.abs() {
            if self.x > 0 {
                Coordinate::new(1, 0)
            } else if self.x < 0 {
                Coordinate::new(-1, 0)
            } else {
                // because 
                // y.abs() >= 0
                // x.abs() > y.abs()
                // =>
                // x.abs() >= y.abs() + 1
                // =>
                // x.abs() >= 0 + 1
                // =>
                // x > 1 || x < 1
                unreachable!()
            }
        } else {
            if self.y > 0 {
                Coordinate::new(0, 1)
            } else if self.y < 0 {
                Coordinate::new(0, -1)
            } else {
                Coordinate::new(0, 0)
            }
        }
    }
}

impl Add for Coordinate {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Coordinate::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Neg for Coordinate {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Coordinate::new(-self.x, -self.y)
    }
}

impl Sub for Coordinate {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl AddAssign for Coordinate {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

#[cfg(test)]
mod coordinate_tests {
    use super::*;

    #[test]
    fn test1() {
        let mut expected_index = 0;
        for y in -4..=4 {
            for x in -4..=4 {
                let c = Coordinate::new(x, y);
                assert_eq!(c.to_index(), Some(expected_index));
                assert_eq!(Coordinate::from_index(expected_index), Some(c));
                expected_index += 1;
            }
        }
    }

    #[test]
    fn test2() {
        let c1 = Coordinate::new(0, 3).orientate_north(Orientation::East);
        assert_eq!(c1, Coordinate::new(3, 0));

        let c1 = Coordinate::new(3, 3).orientate_north(Orientation::East);
        assert_eq!(c1, Coordinate::new(3, -3));

        let c1 = Coordinate::new(3, 3).orientate_north(Orientation::South);
        assert_eq!(c1, Coordinate::new(-3, -3));

        let c1 = Coordinate::new(-4, 2).orientate_north(Orientation::West);
        assert_eq!(c1, Coordinate::new(-3, -3));
    }

    #[test]
    fn test3() {
        let c1 = Coordinate::new(1, 0);
        assert_eq!(Coordinate::new(0, -1), c1.rotate_right());
        assert_eq!(Coordinate::new(-1, 0), c1.rotate_right().rotate_right());
        assert_eq!(Coordinate::new(0, 1), c1.rotate_right().rotate_right().rotate_right());
        assert_eq!(Coordinate::new(1, 0), c1.rotate_right().rotate_right().rotate_right().rotate_right());

        let c1 = Coordinate::new(0, 3);
        assert_eq!(Coordinate::new(3, 0), c1.rotate_right());

    }

    #[test]
    fn test4() {
        let c = Coordinate::new(1, -2) + Coordinate::new(3, 1);
        assert_eq!(c, Coordinate::new(4, -1));
    }
}
