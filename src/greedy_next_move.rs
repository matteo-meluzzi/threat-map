use crate::{direction::Direction, robot_position::RobotPosition, ThreatMap};

pub fn greedy_next_move(threat_map: &ThreatMap, robot_position: &RobotPosition) -> Option<Direction> {
    let mut best_direction = None;
    let mut best_direction_survival_chance = threat_map.at(robot_position.position);

    for direciton in [Direction::Right, Direction::Left, Direction::Front, Direction::Back] {
        let survival_chance = threat_map.at(robot_position.in_direction(direciton));
        if survival_chance >= best_direction_survival_chance {
            best_direction = Some(direciton);
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
        let mut threat_map = ThreatMap::new();
        let bot_coords = [Coordinate::new(-1, -1)];
        threat_map.calculate(&bot_coords);

        let mov = greedy_next_move(&threat_map, &RobotPosition { position: Coordinate::new(0, 0), orientation: Orientation::West });
        assert_eq!(mov, Some(Direction::Back));
    }
}
