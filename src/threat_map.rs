use crate::coordinates::Coordinate;
use crate::N;
use crate::orientation::Orientation;

pub struct ThreatMap {
    map: [i32; N * N],
    pub orientation: Orientation,
}

impl ThreatMap {
    pub fn new(orientation: Orientation) -> Self {
        ThreatMap { map: [i32::MAX; N * N], orientation }
    }

    fn reset(&mut self) {
        self.map.fill(i32::MAX);
    }

    pub fn at(&self, coords: Coordinate, bot_orientation: Orientation) -> i32 {
        let direction = bot_orientation.relative_to(self.orientation);
        match coords.rotate(&direction).to_index() {
            Some(index) => self.map[index],
            None => 0
        }
    }

    pub fn update_orientation(&mut self, orientation: Orientation) {
        self.orientation = orientation;
    }

    pub fn calculate(&mut self, bot_coords: &[Coordinate]) {
        self.reset();

        for index in 0..(N * N) {
            let current_coord = Coordinate::from_index(index).unwrap();
            for bot_coord in bot_coords {
                let distance = current_coord.distance(bot_coord);
                self.map[index] = self.map[index].min(distance)
            }
        }
    }

    pub fn mask_border(&mut self, is_border: &impl Fn(Coordinate) -> bool) {
        for index in 0..(N * N) {
            let current_coord = Coordinate::from_index(index).unwrap();
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
        let mut threat_map = ThreatMap::new(Orientation::North);
        for y in -1..=1 {
            for x in -1..=1 {
                assert_eq!(threat_map.at(Coordinate::new(x, y), Orientation::North), i32::MAX);
            }
        }

        let bot_coords = [Coordinate::new(0, 0)];
        threat_map.calculate(&bot_coords);

        assert_eq!(threat_map.at(Coordinate::new(0, 0), Orientation::North), 0);

        assert_eq!(threat_map.at(Coordinate::new(-1, 0), Orientation::North), 1);
        assert_eq!(threat_map.at(Coordinate::new(0, -1), Orientation::North), 1);
        assert_eq!(threat_map.at(Coordinate::new(0, 1), Orientation::South), 1);
        assert_eq!(threat_map.at(Coordinate::new(0, 1), Orientation::North), 1);
        assert_eq!(threat_map.at(Coordinate::new(1, 0), Orientation::North), 1);

        assert_eq!(threat_map.at(Coordinate::new(-1, -1), Orientation::North), 2);
        assert_eq!(threat_map.at(Coordinate::new(1, 1), Orientation::North), 2);
        assert_eq!(threat_map.at(Coordinate::new(1, -1), Orientation::North), 2);
        assert_eq!(threat_map.at(Coordinate::new(-1, 1), Orientation::North), 2);

        let bot_coords = [Coordinate::new(-1, 1)];
        threat_map.calculate(&bot_coords);

        assert_eq!(threat_map.at(Coordinate::new(-1, 1), Orientation::North), 0);

        assert_eq!(threat_map.at(Coordinate::new(0, 1), Orientation::North), 1);
        assert_eq!(threat_map.at(Coordinate::new(-1, 0), Orientation::North), 1);

        assert_eq!(threat_map.at(Coordinate::new(0, 0), Orientation::North), 2);
        assert_eq!(threat_map.at(Coordinate::new(-1, -1), Orientation::North), 2);
        assert_eq!(threat_map.at(Coordinate::new(1, 1), Orientation::North), 2);

        assert_eq!(threat_map.at(Coordinate::new(1, 0), Orientation::North), 3);
        assert_eq!(threat_map.at(Coordinate::new(0, -1), Orientation::North), 3);

        assert_eq!(threat_map.at(Coordinate::new(1, -1), Orientation::North), 4);
    }
}
