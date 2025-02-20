use crate::{coordinates, ThreatMap, Move};

pub fn greedy_next_move(threat_map: &ThreatMap) -> Option<Move> {
    let mut best_move = None;
    let mut best_move_survival_chance = threat_map.at(coordinates::center_bot());

    for (coords, mov) in [(coordinates::left(), Move::Left), (coordinates::right(), Move::Right)] {
        let survival_chance = threat_map.at(coords); // It takes one move to rotate and one more to move
        if survival_chance > best_move_survival_chance {
            best_move = Some(mov);
            best_move_survival_chance = survival_chance;
        }
    }
    for (coords, mov) in [(coordinates::front(), Move::Front), (coordinates::back(), Move::Back)] {
        let survival_chance = threat_map.at(coords);
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
        let mut threat_map = ThreatMap::new();
        let bot_coords = [BotCentricCoordinate::new(-1, -1)];
        threat_map.calculate(&bot_coords);

        let mov = greedy_next_move(&threat_map);
        assert!(mov == Some(Move::Back));
    }
}
