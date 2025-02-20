use crate::{coordinates, direction::Direction, orientation::Orientation, ThreatMap};

pub fn greedy_next_move(threat_map: &ThreatMap, bot_orientation: Orientation) -> Option<Direction> {
    let mut best_move = None;
    let mut best_move_survival_chance = threat_map.at(coordinates::center_bot(), bot_orientation);

    for (coords, mov) in [(coordinates::front(), Direction::Front), (coordinates::back(), Direction::Back)] {
        let survival_chance = threat_map.at(coords, bot_orientation);
        if survival_chance > best_move_survival_chance {
            best_move = Some(mov);
            best_move_survival_chance = survival_chance;
        }
    }
    for (coords, mov) in [(coordinates::left(), Direction::Left), (coordinates::right(), Direction::Right)] {
        let survival_chance = threat_map.at(coords, bot_orientation); // It takes one move to rotate and one more to move
        if survival_chance > best_move_survival_chance {
            best_move = Some(mov);
            best_move_survival_chance = survival_chance;
        }
    }
    best_move
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::coordinates::BotCentricCoordinate;

    #[test]
    fn test1() {
        let mut threat_map = ThreatMap::new(Orientation::South);
        let bot_coords = [BotCentricCoordinate::new(-1, -1)];
        threat_map.calculate(&bot_coords);

        let mov = greedy_next_move(&threat_map, Orientation::West);
        assert_eq!(mov, Some(Direction::Back));
    }
}
