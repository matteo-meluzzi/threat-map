use crate::{coordinates::Coordinate, N};

#[derive(Debug, Clone)]
pub struct Borders {
    borders: [[bool; N]; N]
}

impl Borders {
    pub fn new() -> Self {
        Self { borders: [[false; N]; N]}
    }

    fn border_indices(&self, coord: Coordinate) -> (usize, usize) {
        let x = coord.x + (N as i8)/2;
        let y = coord.y + (N as i8)/2;
        (x as usize, y as usize)
    }

    pub fn is_border(&self, coord: Coordinate) -> bool {
        if coord.x < -(N as i8)/2 ||
           coord.x >  (N as i8)/2 ||
           coord.y < -(N as i8)/2 ||
           coord.y >  (N as i8)/2 {
            return false;
        }
        let (x, y) = self.border_indices(coord);
        self.borders[x][y]
    }

    pub fn set_border(&mut self, coord: Coordinate) {
        let (x, y) = self.border_indices(coord);
        self.borders[x][y] = true;
    }
}