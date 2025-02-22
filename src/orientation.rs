use crate::direction::Direction;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Orientation {
    North, South, East, West
}

impl Orientation {
    pub fn integer_value(&self) -> i32 {
        match self {
            Self::North => 0,
            Self::East => 1,
            Self::South => 2,
            Self::West => 3
        }
    }

    pub fn from_integer(i: i32) -> Option<Self> {
        match i {
            0 => Some(Self::North),
            1 => Some(Self::East),
            2 => Some(Self::South),
            3 => Some(Self::West),
            _ => None
        }
    }

    pub fn direction_relative_to(&self, other: Orientation) -> Direction {
        match self.integer_value() - other.integer_value() {
            3 => Direction::Left,
            2 => Direction::Back,
            1 => Direction::Right,
            0 => Direction::Front,
            -1 => Direction::Left,
            -2 => Direction::Back,
            -3 => Direction::Right,
            _ => unreachable!()
        }
    }

    pub fn rotated_right(&self) -> Orientation {
        match self {
            Orientation::North => Orientation::East,
            Orientation::East => Orientation::South,
            Orientation::South => Orientation::West,
            Orientation::West => Orientation::North
        }
    }

    pub fn rotated_left(&self) -> Orientation {
        match self {
            Orientation::North => Orientation::West,
            Orientation::East => Orientation::North,
            Orientation::South => Orientation::East,
            Orientation::West => Orientation::South
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Orientation::West.direction_relative_to(Orientation::North), Direction::Left);
        assert_eq!(Orientation::South.direction_relative_to(Orientation::North), Direction::Back);
        assert_eq!(Orientation::West.direction_relative_to(Orientation::East), Direction::Back);
        assert_eq!(Orientation::North.direction_relative_to(Orientation::West), Direction::Right);
    }
}
