use crate::connection::Connection;

pub struct MazeCell {
    pub x: usize,
    pub y: usize,
    pub n_wall: bool,
    pub w_wall: bool,
    pub s_wall: bool,
    pub e_wall: bool,
}

impl MazeCell {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            n_wall: true,
            w_wall: true,
            s_wall: true,
            e_wall: true,
        }
    }

    pub fn process_connection(&mut self, connection: &Connection) {
        let other: (usize, usize) = if connection.x0 == self.x && connection.y0 == self.y {
            (connection.x1, connection.y1)
        } else {
            (connection.x0, connection.y0)
        };
        if self.x == other.0 {
            if self.y > other.1 {
                self.s_wall = false;
            } else {
                self.n_wall = false;
            }
        } else if self.x > other.0 {
            self.e_wall = false;
        } else {
            self.w_wall = false;
        }
    }
}
