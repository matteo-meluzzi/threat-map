#![no_std]

use threat_map::ThreatMap;

pub mod coordinates;
pub mod threat_map;
pub mod greedy_next_move;
pub mod direction;
pub mod orientation;

pub const N: usize = 9;
