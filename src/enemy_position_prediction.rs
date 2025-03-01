use crate::{borders::Borders, coordinates::Coordinate, enemy_position::EnemyPositions, MAX_NUM_ENEMIES, N};

#[derive(Debug, Clone, Copy)]
struct Enemy { pub position: Coordinate, pub direction: Coordinate }

pub struct EnemyPositionPrediction {
    future_positions_mem: [Enemy; MAX_NUM_ENEMIES],
    future_positions_count: usize,
    borders: Borders
}

impl EnemyPositionPrediction {
    pub fn empty() -> Self {
        EnemyPositionPrediction { future_positions_mem: [Enemy{position: Coordinate::new(0, 0), direction: Coordinate::new(0, 0)}; MAX_NUM_ENEMIES], future_positions_count: 0, borders: Borders::new() }
    }

    pub fn new(current_positions: &EnemyPositions, previous_positions: &EnemyPositions, borders: Borders) -> Self {
        let mut future_positions = [Enemy{position: Coordinate::new(0, 0), direction: Coordinate::new(0, 0)}; MAX_NUM_ENEMIES];
        let mut future_positions_count = 0;

        for curr_e in current_positions {
            future_positions[future_positions_count].position = curr_e.position;

            if let Some(prev_e_position) = previous_positions.get_position_of(curr_e.id) {
                let direction = (curr_e.position - prev_e_position).normalized();
                future_positions[future_positions_count].direction = direction;
            } else if curr_e.position.is_corner() { // enemies in the corner are assumed to stand still
                future_positions[future_positions_count].direction = Coordinate::new(0, 0);
            } else if curr_e.position.y == -(N as i8)/2 { // assume that enemies at the border will go towards the center.
                future_positions[future_positions_count].direction = Coordinate::new(0, 1);
            } else if curr_e.position.y == (N as i8)/2 { 
                future_positions[future_positions_count].direction = Coordinate::new(0, -1);
            } else if curr_e.position.x == -(N as i8)/2 {
                future_positions[future_positions_count].direction = Coordinate::new(1, 0);
            } else if curr_e.position.x == (N as i8)/2 {
                future_positions[future_positions_count].direction = Coordinate::new(-1, 0);
            }

            future_positions_count += 1
        }
        
        EnemyPositionPrediction{ future_positions_mem: future_positions, future_positions_count, borders }
    }

    fn future_positions(&self) -> &[Enemy] {
        &self.future_positions_mem[..self.future_positions_count]
    }

    pub fn move_enemies(&mut self) {
        for enemy in &mut self.future_positions_mem[..self.future_positions_count] {
            // enemies bounce on borders
            if self.borders.is_border(enemy.position + enemy.direction) {
                enemy.direction = -enemy.direction;
            }
            enemy.position += enemy.direction
        }
    }

    pub fn min_distance_from(&self, from: Coordinate) -> i32 {
        self.future_positions().iter().map(|enemy| enemy.position.distance(from)).min().unwrap_or(0)
    }
}

#[cfg(test)]
mod prediction_tests {
    use super::*;

    #[test]
    fn test1() {

    }
}