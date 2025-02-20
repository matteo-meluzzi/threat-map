#![no_std]
#![feature(generic_const_exprs)]

use threat_map::ThreatMap;

pub mod coordinates;
pub mod threat_map;
pub mod greedy_next_move;

pub const N: usize = 9;

#[derive(Debug, PartialEq, Eq)]
pub enum Orientation {
    North, South, East, West
}

#[derive(Debug, PartialEq, Eq)]
pub enum Move {
    Front, Back, Right, Left
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
