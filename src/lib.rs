#![no_std]

pub mod coordinates;
pub mod threat_map;
pub mod greedy_next_move;
pub mod direction;
pub mod orientation;
pub mod robot_position;
pub mod enemy_position;
pub mod enemy_position_prediction;
pub mod borders;

pub const N: usize = 9;
pub const MAX_NUM_ENEMIES: usize = 10;
