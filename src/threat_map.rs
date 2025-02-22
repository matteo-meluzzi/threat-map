use crate::coordinates::Coordinate;
use crate::enemy_position::EnemyPositions;
use crate::N;

pub struct ThreatMap {
    map: [i32; N * N],
}

impl ThreatMap {
    pub fn new() -> Self {
        ThreatMap { map: [i32::MAX; N * N] }
    }

    fn reset(&mut self) {
        self.map.fill(i32::MAX);
    }

    pub fn at(&self, coords: Coordinate) -> i32 {
        match coords.to_index() {
            Some(i) => self.map[i],
            None => 0
        }
    }

    fn calculate(&mut self, bot_coords: &[Coordinate]) {
        self.reset();

        for index in 0..(N * N) {
            let current_coord = Coordinate::from_index(index).unwrap();
            for &bot_coord in bot_coords {
                let distance = current_coord.distance(bot_coord);
                self.map[index] = self.map[index].min(distance)
            }
        }
    }

    pub fn calculate_with_previous_location(&mut self, current_enemy_positions: &EnemyPositions, previous_enemy_positions: &EnemyPositions) {
        let mut future_enemy_positions = [Coordinate::new(0, 0); N * N];
        let mut future_enemy_count = 0;
        for current_position in current_enemy_positions {
            match previous_enemy_positions.get_position_of(current_position.id) {
                Some(previous_position) => {
                    let future_position = current_position.position + current_position.position - previous_position;
                    // we take the worst case scenario of the closest point to us
                    if future_position.distance(Coordinate::new(0, 0)) < current_position.position.distance(Coordinate::new(0, 0)) { 
                        future_enemy_positions[future_enemy_count] = future_position;
                    } else {
                        future_enemy_positions[future_enemy_count] = current_position.position;
                    }
                    future_enemy_count += 1;
                }
                None => {
                    future_enemy_positions[future_enemy_count] = current_position.position;
                    future_enemy_count += 1;
                }
            }
        }

        self.calculate(&future_enemy_positions[..future_enemy_count]);
    }

    pub fn mask_border(&mut self, border_coord: Coordinate) {
        if let Some(index) = border_coord.to_index() {
            self.map[index] = 0;
        }
    }
}

#[cfg(test)]
mod threat_map_tests {
    use super::*;

    #[test]
    fn test1() {
        let mut threat_map = ThreatMap::new();
        for y in -1..=1 {
            for x in -1..=1 {
                assert_eq!(threat_map.at(Coordinate::new(x, y)), i32::MAX);
            }
        }

        let bot_coords = [Coordinate::new(0, 0)];
        threat_map.calculate(&bot_coords);

        assert_eq!(threat_map.at(Coordinate::new(0, 0)), 0);

        assert_eq!(threat_map.at(Coordinate::new(-1, 0)), 1);
        assert_eq!(threat_map.at(Coordinate::new(0, -1)), 1);
        assert_eq!(threat_map.at(Coordinate::new(0, 1)), 1);
        assert_eq!(threat_map.at(Coordinate::new(1, 0)), 1);

        assert_eq!(threat_map.at(Coordinate::new(-1, -1)), 2);
        assert_eq!(threat_map.at(Coordinate::new(1, 1)), 2);
        assert_eq!(threat_map.at(Coordinate::new(1, -1)), 2);
        assert_eq!(threat_map.at(Coordinate::new(-1, 1)), 2);

        let bot_coords = [Coordinate::new(-1, 1)];
        threat_map.calculate(&bot_coords);

        assert_eq!(threat_map.at(Coordinate::new(-1, 1)), 0);

        assert_eq!(threat_map.at(Coordinate::new(0, 1)), 1);
        assert_eq!(threat_map.at(Coordinate::new(-1, 0)), 1);

        assert_eq!(threat_map.at(Coordinate::new(0, 0)), 2);
        assert_eq!(threat_map.at(Coordinate::new(-1, -1)), 2);
        assert_eq!(threat_map.at(Coordinate::new(1, 1)), 2);

        assert_eq!(threat_map.at(Coordinate::new(1, 0)), 3);
        assert_eq!(threat_map.at(Coordinate::new(0, -1)), 3);

        assert_eq!(threat_map.at(Coordinate::new(1, -1)), 4);
    }
}
