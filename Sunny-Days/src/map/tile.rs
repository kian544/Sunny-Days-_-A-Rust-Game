#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Wall,
    Floor,
    DoorForward, // leads to a new map
    DoorBack,    // leads to previous map
}
