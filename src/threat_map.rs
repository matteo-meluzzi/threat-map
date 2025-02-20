use crate::coordinates::BotCentricCoordinate;
use crate::N;

pub struct ThreatMap {
    map: [i32; N * N] 
}

impl ThreatMap {
    pub fn new() -> Self {
        ThreatMap { map: [i32::MAX; N * N] }
    }

    fn reset(&mut self) {
        self.map.fill(i32::MAX);
    }

    pub fn at(&self, coords: BotCentricCoordinate) -> i32 {
        match coords.to_index() {
            Some(index) => self.map[index],
            None => 0
        }
    }

    pub fn calculate(&mut self, bot_coords: &[BotCentricCoordinate]) {
        self.reset();

        for index in 0..(N * N) {
            let current_coord = BotCentricCoordinate::from_index(index).unwrap();
            for bot_coord in bot_coords {
                let distance = current_coord.distance(bot_coord);
                self.map[index] = self.map[index].min(distance)
            }
        }
    }

    pub fn mask_border(&mut self, is_border: &impl Fn(BotCentricCoordinate) -> bool) {
        for index in 0..(N * N) {
            let current_coord = BotCentricCoordinate::from_index(index).unwrap();
            if is_border(current_coord) {
                self.map[index] = 0;
            }
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
                assert_eq!(threat_map.at(BotCentricCoordinate::new(x, y)), i32::MAX);
            }
        }

        let bot_coords = [BotCentricCoordinate::new(0, 0)];
        threat_map.calculate(&bot_coords);

        assert_eq!(threat_map.at(BotCentricCoordinate::new(0, 0)), 0);

        assert_eq!(threat_map.at(BotCentricCoordinate::new(-1, 0)), 1);
        assert_eq!(threat_map.at(BotCentricCoordinate::new(0, -1)), 1);
        assert_eq!(threat_map.at(BotCentricCoordinate::new(0, 1)), 1);
        assert_eq!(threat_map.at(BotCentricCoordinate::new(1, 0)), 1);

        assert_eq!(threat_map.at(BotCentricCoordinate::new(-1, -1)), 2);
        assert_eq!(threat_map.at(BotCentricCoordinate::new(1, 1)), 2);
        assert_eq!(threat_map.at(BotCentricCoordinate::new(1, -1)), 2);
        assert_eq!(threat_map.at(BotCentricCoordinate::new(-1, 1)), 2);

        let bot_coords = [BotCentricCoordinate::new(-1, 1)];
        threat_map.calculate(&bot_coords);

        assert_eq!(threat_map.at(BotCentricCoordinate::new(-1, 1)), 0);

        assert_eq!(threat_map.at(BotCentricCoordinate::new(0, 1)), 1);
        assert_eq!(threat_map.at(BotCentricCoordinate::new(-1, 0)), 1);

        assert_eq!(threat_map.at(BotCentricCoordinate::new(0, 0)), 2);
        assert_eq!(threat_map.at(BotCentricCoordinate::new(-1, -1)), 2);
        assert_eq!(threat_map.at(BotCentricCoordinate::new(1, 1)), 2);

        assert_eq!(threat_map.at(BotCentricCoordinate::new(1, 0)), 3);
        assert_eq!(threat_map.at(BotCentricCoordinate::new(0, -1)), 3);

        assert_eq!(threat_map.at(BotCentricCoordinate::new(1, -1)), 4);
    }
}
