use crate::map::Map;

#[derive(Debug, Clone, Copy)]
pub struct Player {
    pub x: i32,
    pub y: i32,
    pub hp: i32,
}

impl Player {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y, hp: 30 }
    }

    pub fn try_move(&mut self, dx: i32, dy: i32, map: &Map) {
        let nx = self.x + dx;
        let ny = self.y + dy;

        if nx < 0 || ny < 0 || nx >= map.width as i32 || ny >= map.height as i32 {
            return;
        }

        if map.is_walkable(nx as usize, ny as usize) {
            self.x = nx;
            self.y = ny;
        }

    }
}
