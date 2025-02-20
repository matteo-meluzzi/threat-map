use crate::{direction::Direction, N};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub struct BotCentricCoordinate {
    pub x: i8,
    pub y: i8
}

impl BotCentricCoordinate {
    pub fn new(x: i8, y: i8) -> Self {
        let bound = N as i8 / 2;
        assert!(x >= -bound);
        assert!(y >= -bound);
        assert!(x <= bound);
        assert!(y <= bound);

        BotCentricCoordinate { x, y }
    }

    pub fn from_index(index: usize) -> Option<Self> {        
        let center = N as i32 / 2;
        let row = index as i32 / N as i32;
        let col = index as i32 % N as i32;

        let x = col - center;
        let y = row - center;
        
        let bound = N as i32 / 2;
        if x >= -bound && y >= -bound && x <= bound && y <= bound {
            Some(BotCentricCoordinate{ x: x as i8, y: y as i8 })
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

    pub fn distance(&self, other: &BotCentricCoordinate) -> i32 {
        self.x.abs_diff(other.x) as i32 + self.y.abs_diff(other.y) as i32
    }

    fn rotate_right(&self) -> BotCentricCoordinate {
        BotCentricCoordinate { x: self.y, y: -self.x }
    }

    pub fn rotate(&self, direction: &Direction) -> BotCentricCoordinate {
        match direction {
            Direction::Front => *self,
            Direction::Right => self.rotate_right(),
            Direction::Back => self.rotate_right().rotate_right(),
            Direction::Left => self.rotate_right().rotate_right().rotate_right()
        }
    }
}

pub fn front() -> BotCentricCoordinate {
    return BotCentricCoordinate::new(0, -1);
}
pub fn back() -> BotCentricCoordinate {
    return BotCentricCoordinate::new(0, 1);
}
pub fn left() -> BotCentricCoordinate {
    return BotCentricCoordinate::new(-1, 0);
}
pub fn right() -> BotCentricCoordinate {
    return BotCentricCoordinate::new(1, 0);
}
pub fn center_bot() -> BotCentricCoordinate {
    return BotCentricCoordinate::new(0, 0);
}

#[cfg(test)]
mod bot_centric_coordinate_tests {
    use super::*;

    #[test]
    fn test1() {
        let mut expected_index = 0;
        for y in -4..=4 {
            for x in -4..=4 {
                let c = BotCentricCoordinate::new(x, y);
                assert_eq!(c.to_index(), Some(expected_index));
                assert_eq!(BotCentricCoordinate::from_index(expected_index), Some(c));
                expected_index += 1;
            }
        }

        // let mut expected_index = 0;
        // for y in -1..=1 {
        //     for x in -1..=1 {
        //         let c = BotCentricCoordinate::new(x, y);
        //         assert_eq!(c.to_index(), Some(expected_index));
        //         assert_eq!(BotCentricCoordinate::from_index(expected_index), Some(c));
        //         expected_index += 1;
        //     }
        // }

        // assert_eq!(BotCentricCoordinate::<3>::from_index(10), None);
        // assert_eq!(BotCentricCoordinate::<3>{x: 2, y: 0}.to_index(), None);
    }
}
