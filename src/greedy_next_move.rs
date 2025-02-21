use crate::{coordinates::Coordinate, direction::Direction, orientation::Orientation, ThreatMap};

pub fn greedy_next_move(threat_map: &ThreatMap, bot_orientation: Orientation, bot_position: Coordinate) -> Option<Direction> {
    let mut best_direction = None;
    let mut best_direction_survival_chance = threat_map.at(bot_position, bot_orientation);

    for direciton in [Direction::Front, Direction::Back] {
        let survival_chance = threat_map.at(bot_position.in_direction(direciton), bot_orientation);
        if survival_chance > best_direction_survival_chance {
            best_direction = Some(direciton);
            best_direction_survival_chance = survival_chance;
        }
    }
    for direciton in [Direction::Right, Direction::Left] {
        let survival_chance = threat_map.at(bot_position.in_direction(direciton), bot_orientation); // It takes one move to rotate and one more to move
        if survival_chance > best_direction_survival_chance {
            best_direction = Some(direciton);
            best_direction_survival_chance = survival_chance;
        }
    }
    best_direction
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut threat_map = ThreatMap::new(Orientation::South);
        let bot_coords = [Coordinate::new(-1, -1)];
        threat_map.calculate(&bot_coords);

        let mov = greedy_next_move(&threat_map, Orientation::West, Coordinate { x: 0, y: 0 });
        assert_eq!(mov, Some(Direction::Back));
    }
}
