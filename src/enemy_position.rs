use core::num::NonZero;

use crate::coordinates::Coordinate;
use crate::{MAX_NUM_ENEMIES, N};

#[derive(Debug, Clone, Copy)]
pub struct EnemyPosition {
    pub id: NonZero<u64>,
    pub position: Coordinate,
}

impl EnemyPosition {
    pub fn new(id: NonZero<u64>, position: Coordinate) -> Self {
        Self { id, position }
    }
}

#[derive(Debug, Clone)]
pub struct EnemyPositions {
    enemy_positions: [EnemyPosition; MAX_NUM_ENEMIES],
    length: usize,
    origin: Coordinate,
}

impl EnemyPositions {
    pub fn new() -> Self {
        EnemyPositions { enemy_positions: [EnemyPosition::new(NonZero::new(1).unwrap(), Coordinate::new(0, 0)); MAX_NUM_ENEMIES], length: 0, origin: Coordinate::new(0, 0) }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn clear(&mut self) {
        self.length = 0;
    }

    pub fn push(&mut self, enemy_position: EnemyPosition) {
        if self.len() == MAX_NUM_ENEMIES {
            return;
        }
        self.enemy_positions[self.len()] = enemy_position;
        self.length += 1;
    }

    pub fn use_origin(&mut self, origin: Coordinate) {
        self.origin = origin
    }

    pub fn get_position_of(&self, id: NonZero<u64>) -> Option<Coordinate> {
        let enemy_position = self.enemy_positions.iter().find(|&e| e.id == id).copied()?;
        Some(enemy_position.position - self.origin)
    }

    pub fn iter(&self) -> core::slice::Iter<EnemyPosition> {
        self.enemy_positions[..self.len()].iter()
    }

    pub fn iter_mut(&mut self) -> core::slice::IterMut<EnemyPosition> {
        let len: usize = self.len();
        self.enemy_positions[..len].iter_mut()
    }
}

impl<'a> IntoIterator for &'a EnemyPositions {
    type Item = &'a EnemyPosition;

    type IntoIter = core::slice::Iter<'a, EnemyPosition>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut EnemyPositions {
    type Item = &'a mut EnemyPosition;

    type IntoIter = core::slice::IterMut<'a, EnemyPosition>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

#[cfg(test)]
mod test {
    use crate::{orientation::Orientation, robot_position::RobotPosition};

    use super::*;

    #[test]
    fn test1() {
        let mut old_enemy_positions = EnemyPositions::new();
        old_enemy_positions.push(EnemyPosition::new(1, Coordinate::new(0, 0)));

        let mut current_enemy_positions = EnemyPositions::new();
        current_enemy_positions.push(EnemyPosition::new(1, Coordinate::new(1, 0)));
        current_enemy_positions.push(EnemyPosition::new(2, Coordinate::new(2, 0)));

        let robot_position = RobotPosition{position: Coordinate::new(2, 0), orientation: Orientation::East};
        old_enemy_positions.use_origin(robot_position.position);
        
        let old_1 = old_enemy_positions.get_position_of(1).unwrap();
        let curr_1 = current_enemy_positions.get_position_of(1).unwrap();

        assert_eq!(old_1.distance(curr_1), 3);
    }

    #[test]
    fn test2() {
        let mut old_enemy_positions = EnemyPositions::new();
        for i in &old_enemy_positions {

        }
    }
}