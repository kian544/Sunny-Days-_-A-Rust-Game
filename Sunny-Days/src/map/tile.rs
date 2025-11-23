#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Wall,
    Floor,
    Door, // single door used to toggle between Room 1 and Room 2
}
