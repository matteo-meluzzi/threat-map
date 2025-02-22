use core::fmt::{Debug, Write};

use crate::coordinates::Coordinate;
use crate::N;

pub struct ThreatMap {
    map: [i32; N * N],
}

impl Debug for ThreatMap {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for index in 0..(N * N) {
            let current_coord = Coordinate::from_index(index).unwrap();
            f.write_fmt(format_args!("{} ", self.map[index]))?;
            if current_coord.x == N as i8 - 1 {
                f.write_char('\n')?;
            } 
        }  

        core::fmt::Result::Ok(())
    }
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
