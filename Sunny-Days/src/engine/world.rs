use crate::engine::action::Action;
use crate::engine::entity::Player;
use crate::map::{generator::generate_rooms_and_corridors, tile::Tile, Map};

use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

use std::collections::VecDeque;

#[derive(Clone)]
pub struct Level {
    pub map: Map,
    pub forward_door: (i32, i32), // door to go deeper
    pub back_door: Option<(i32, i32)>, // door to go back (none on level 0)
}

pub struct World {
    pub levels: Vec<Level>,      // stack of levels
    pub player: Player,
    pub logs: VecDeque<String>,
    pub seed: u64,
    pub width: usize,
    pub height: usize,
}

impl World {
    pub fn new(seed: u64, width: usize, height: usize) -> Self {
        let (level0, spawn) = Self::make_level(seed, 0, width, height, None);

        let mut logs = VecDeque::new();
        logs.push_back(format!("Seed: {}", seed));
        logs.push_back("Welcome to the dungeon.".to_string());
        logs.push_back("Move with WASD or arrow keys. Press Q to quit.".to_string());
        logs.push_back("Find the white door to go deeper.".to_string());

        Self {
            levels: vec![level0],
            player: Player::new(spawn.0, spawn.1),
            logs,
            seed,
            width,
            height,
        }
    }

    fn current_level(&self) -> &Level {
        self.levels.last().expect("no levels")
    }

    fn current_level_mut(&mut self) -> &mut Level {
        self.levels.last_mut().expect("no levels")
    }

    pub fn current_map(&self) -> &Map {
        &self.current_level().map
    }

    pub fn current_map_mut(&mut self) -> &mut Map {
        &mut self.current_level_mut().map
    }

    pub fn push_log(&mut self, msg: impl Into<String>) {
        self.logs.push_back(msg.into());
        while self.logs.len() > 6 {
            self.logs.pop_front();
        }
    }

    /// Creates a level, places a forward door randomly,
    /// and optionally places a back door at a specific position.
    fn make_level(
        base_seed: u64,
        depth: usize,
        width: usize,
        height: usize,
        back_door_at: Option<(i32, i32)>,
    ) -> (Level, (i32, i32)) {
        let seed = base_seed.wrapping_add(depth as u64 * 9_973);
        let mut map = generate_rooms_and_corridors(width, height, seed);

        // spawn = first floor tile
        let (sx, sy) = map.find_first_floor().unwrap_or((1, 1));
        let spawn = (sx as i32, sy as i32);

        // collect all floor tiles (candidates for forward door)
        let mut floors: Vec<(i32, i32)> = Vec::new();
        for y in 0..height {
            for x in 0..width {
                if map.get(x, y) == Tile::Floor {
                    floors.push((x as i32, y as i32));
                }
            }
        }

        let mut rng = StdRng::seed_from_u64(seed ^ 0xA11CE);

        // choose random floor for forward door, excluding spawn
        let mut forward_door = spawn;
        if floors.len() > 1 {
            loop {
                let idx = rng.gen_range(0..floors.len());
                let candidate = floors[idx];
                if candidate != spawn {
                    forward_door = candidate;
                    break;
                }
            }
        }

        // place forward door tile
        map.set(forward_door.0 as usize, forward_door.1 as usize, Tile::DoorForward);

        // place back door if requested
        let back_door = back_door_at;
        if let Some((bx, by)) = back_door {
            map.set(bx as usize, by as usize, Tile::DoorBack);
        }

        (
            Level {
                map,
                forward_door,
                back_door,
            },
            spawn,
        )
    }

    fn enter_new_level(&mut self) {
        let depth = self.levels.len(); // new depth

        // When entering new level, back door will be at the new spawn position.
        let (new_level, spawn) =
            Self::make_level(self.seed, depth, self.width, self.height, Some((0,0)));

        // We set back door at spawn, but we only know spawn after generation.
        // So patch it now:
        let mut patched_level = new_level.clone();
        let (sx, sy) = (spawn.0 as usize, spawn.1 as usize);
        patched_level.map.set(sx, sy, Tile::DoorBack);
        patched_level.back_door = Some(spawn);

        self.levels.push(patched_level);

        self.player.x = spawn.0;
        self.player.y = spawn.1;

        self.push_log(format!("Entered level {}", depth));
    }

    fn return_to_prev_level(&mut self) {
        if self.levels.len() <= 1 {
            self.push_log("No previous level to return to.".to_string());
            return;
        }

        // Remove current level
        self.levels.pop();

        // Put player back on the forward door of previous level
        let prev = self.current_level().forward_door;
        self.player.x = prev.0;
        self.player.y = prev.1;

        let depth = self.levels.len() - 1;
        self.push_log(format!("Returned to level {}", depth));
    }

    pub fn apply_action(&mut self, action: Action) -> bool {
        match action {
            Action::Move(dx, dy) => {
                let old = (self.player.x, self.player.y);
                let map = self.current_map().clone(); // snapshot for borrow simplicity
                self.player.try_move(dx, dy, &map);
                let newp = (self.player.x, self.player.y);

                if old != newp {
                    self.push_log(format!("Player moved to ({}, {})", newp.0, newp.1));
                }

                // After moving, check tile under player
                let tile = self.current_map().get(newp.0 as usize, newp.1 as usize);
                match tile {
                    Tile::DoorForward => {
                        self.push_log("You step through the door...".to_string());
                        self.enter_new_level();
                    }
                    Tile::DoorBack => {
                        self.push_log("You return to the previous area.".to_string());
                        self.return_to_prev_level();
                    }
                    _ => {}
                }

                true
            }
            Action::Quit => false,
            Action::None => true,
        }
    }
}
