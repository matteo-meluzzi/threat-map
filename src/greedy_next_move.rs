use crate::{borders::Borders, direction::Direction, enemy_position_prediction::EnemyPositionPrediction, robot_position::RobotPosition};

pub fn greedy_next_move(robot_position: &RobotPosition, enemy_position_prediction: &EnemyPositionPrediction, borders: &Borders) -> Option<Direction> {
    let mut best_direction = None;
    let mut best_direction_survival_chance = enemy_position_prediction.min_distance_from(robot_position.position);

    for direction in [Direction::Right, Direction::Left, Direction::Front, Direction::Back] {
        let pos = robot_position.in_direction(direction);
        // do not go to a border
        if borders.is_border(pos) {
            continue;
        }
        let survival_chance = enemy_position_prediction.min_distance_from(pos);
        if survival_chance >= best_direction_survival_chance {
            best_direction = Some(direction);
            best_direction_survival_chance = survival_chance;
        }
    }
    
    best_direction
}

#[cfg(test)]
mod tests {
    use crate::coordinates::Coordinate;
    use crate::orientation::Orientation;

    use super::*;

    #[test]
    fn test1() {
        // let mut enemies = EnemyPositionPrediction::new();
        // let bot_coords = [Coordinate::new(-1, -1)];
        // threat_map.calculate(&bot_coords);

        // let mov = greedy_next_move(&threat_map, &RobotPosition { position: Coordinate::new(0, 0), orientation: Orientation::West });
        // assert_eq!(mov, Some(Direction::Back));
    }
}
