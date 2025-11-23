use crate::map::{tile::Tile, Map};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

#[derive(Clone, Copy)]
struct Room {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

impl Room {
    fn new(x: usize, y: usize, w: usize, h: usize) -> Self {
        Self { x1: x, y1: y, x2: x + w, y2: y + h }
    }

    fn center(&self) -> (usize, usize) {
        ((self.x1 + self.x2) / 2, (self.y1 + self.y2) / 2)
    }

    fn intersects(&self, other: &Room) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1 &&
        self.y1 <= other.y2 && self.y2 >= other.y1
    }
}

pub fn generate_rooms_and_corridors(width: usize, height: usize, seed: u64) -> Map {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut map = Map::new(width, height, Tile::Wall);

    let mut rooms: Vec<Room> = Vec::new();
    let max_rooms = 14;
    let min_size = 5;
    let max_size = 12;

    for _ in 0..max_rooms {
        let w = rng.gen_range(min_size..=max_size);
        let h = rng.gen_range(min_size..=max_size);

        let x = rng.gen_range(1..width.saturating_sub(w + 2));
        let y = rng.gen_range(1..height.saturating_sub(h + 2));

        let room = Room::new(x, y, w, h);

        if rooms.iter().any(|r| r.intersects(&room)) {
            continue;
        }

        carve_room(&mut map, &room);

        if let Some(prev) = rooms.last() {
            let (newx, newy) = room.center();
            let (prevx, prevy) = prev.center();
            carve_corridor(&mut map, prevx, prevy, newx, newy, &mut rng);
        }

        rooms.push(room);
    }

    map
}

fn carve_room(map: &mut Map, room: &Room) {
    for y in room.y1..room.y2 {
        for x in room.x1..room.x2 {
            map.set(x, y, Tile::Floor);
        }
    }
}

fn carve_corridor(map: &mut Map, x1: usize, y1: usize, x2: usize, y2: usize, rng: &mut impl Rng) {
    if rng.gen_bool(0.5) {
        carve_h(map, x1, x2, y1);
        carve_v(map, y1, y2, x2);
    } else {
        carve_v(map, y1, y2, x1);
        carve_h(map, x1, x2, y2);
    }
}

fn carve_h(map: &mut Map, x1: usize, x2: usize, y: usize) {
    let (start, end) = if x1 <= x2 { (x1, x2) } else { (x2, x1) };
    for x in start..=end {
        map.set(x, y, Tile::Floor);
    }
}

fn carve_v(map: &mut Map, y1: usize, y2: usize, x: usize) {
    let (start, end) = if y1 <= y2 { (y1, y2) } else { (y2, y1) };
    for y in start..=end {
        map.set(x, y, Tile::Floor);
    }
}
