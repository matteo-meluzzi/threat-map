use crate::{coordinates::Coordinate, direction::Direction, orientation::Orientation};

pub struct RobotPosition {
    pub position: Coordinate,
    pub orientation: Orientation,
}

impl RobotPosition {
    pub fn take_step(&mut self, step_direction: Direction) {
        match step_direction {
            Direction::Front => self.position = self.position.in_direction(Direction::Front, self.orientation),
            Direction::Right => self.orientation = self.orientation.rotated_right(),
            Direction::Back =>  self.position = self.position.in_direction(Direction::Back, self.orientation),
            Direction::Left => self.orientation = self.orientation.rotated_left(),
        }
    }
    
    pub fn in_direction(&self, direction: Direction) -> Coordinate {
        self.position.in_direction(direction, self.orientation)
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use crate::orientation::Orientation;
    use crate::threat_map::ThreatMap;

    #[test]
    fn test1() {
        let mut robot_position = RobotPosition{position: Coordinate::new(0, 0), orientation: Orientation::North};
        let mut map: ThreatMap = ThreatMap::new();
        map.calculate(&[Coordinate::new(0,0 )]);
        
        robot_position.take_step(Direction::Front);
        assert_eq!(robot_position.position, Coordinate::new(0, -1));
        assert_eq!(robot_position.orientation, Orientation::North);

        robot_position.take_step(Direction::Right);
        assert_eq!(robot_position.position, Coordinate::new(0, -1));
        assert_eq!(robot_position.orientation, Orientation::East);

        robot_position.take_step(Direction::Front);
        assert_eq!(robot_position.position, Coordinate::new(1, -1));
        assert_eq!(robot_position.orientation, Orientation::East);

        robot_position.take_step(Direction::Back);
        assert_eq!(robot_position.position, Coordinate::new(0, -1));
        assert_eq!(robot_position.orientation, Orientation::East);

        robot_position.take_step(Direction::Left);
        assert_eq!(robot_position.position, Coordinate::new(0, -1));
        assert_eq!(robot_position.orientation, Orientation::North);

        robot_position.take_step(Direction::Left);
        assert_eq!(robot_position.position, Coordinate::new(0, -1));
        assert_eq!(robot_position.orientation, Orientation::West);

        robot_position.take_step(Direction::Front);
        assert_eq!(robot_position.position, Coordinate::new(-1, -1));
        assert_eq!(robot_position.orientation, Orientation::West);

        robot_position.take_step(Direction::Left);
        assert_eq!(robot_position.position, Coordinate::new(-1, -1));
        assert_eq!(robot_position.orientation, Orientation::South);

        robot_position.take_step(Direction::Front);
        assert_eq!(robot_position.position, Coordinate::new(-1, 0));
        assert_eq!(robot_position.orientation, Orientation::South);

        robot_position.take_step(Direction::Front);
        assert_eq!(robot_position.position, Coordinate::new(-1, 1));
        assert_eq!(robot_position.orientation, Orientation::South);

        assert_eq!(map.at(robot_position.position), 2)
    }

    #[test]
    fn test2() {
        let mut robot_position = RobotPosition{position: Coordinate::new(0, 0), orientation: Orientation::East};
        let mut map: ThreatMap = ThreatMap::new();
        let enemy_positions = &mut [Coordinate::new(0,3 )];
        enemy_positions.iter_mut().for_each(|c| *c = c.orientate_north(robot_position.orientation));
        map.calculate(enemy_positions);

        assert_eq!(map.at(Coordinate::new(0, 3)), 6);
        assert_eq!(map.at(Coordinate::new(3, 0)), 0);
        assert_eq!(map.at(Coordinate::new(0, 0)), 3);

        robot_position.take_step(Direction::Front);
        assert_eq!(map.at(robot_position.position), 2);
        assert_eq!(map.at(robot_position.in_direction(Direction::Back)), 3);
        assert_eq!(robot_position.in_direction(Direction::Back), Coordinate::new(0, 0));
        assert_eq!(robot_position.in_direction(Direction::Front), Coordinate::new(2, 0));
        robot_position.take_step(Direction::Front);
        assert_eq!(map.at(robot_position.position), 1);
        robot_position.take_step(Direction::Front);
        assert_eq!(map.at(robot_position.position), 0);
        assert_eq!(map.at(robot_position.in_direction(Direction::Right)), 1);
        assert_eq!(map.at(robot_position.in_direction(Direction::Front)), 1);
    }
}