use crate::engine::action::Action;
use crate::engine::entity::Player;
use crate::map::{generator::generate_rooms_and_corridors, Map};

use std::collections::VecDeque;

pub struct World {
    pub map: Map,
    pub player: Player,
    pub logs: VecDeque<String>,
    pub seed: u64,
}

impl World {
    pub fn new(seed: u64, width: usize, height: usize) -> Self {
        let map = generate_rooms_and_corridors(width, height, seed);

        let (sx, sy) = map.find_first_floor().unwrap_or((1, 1));

        let mut logs = VecDeque::new();
        logs.push_back(format!("Seed: {}", seed));
        logs.push_back("Welcome to the dungeon.".to_string());
        logs.push_back("Move with WASD or arrow keys. Press Q to quit.".to_string());

        Self {
            map,
            player: Player::new(sx as i32, sy as i32),
            logs,
            seed,
        }
    }

    pub fn push_log(&mut self, msg: impl Into<String>) {
        self.logs.push_back(msg.into());
        while self.logs.len() > 6 {
            self.logs.pop_front();
        }
    }

    pub fn apply_action(&mut self, action: Action) -> bool {
        match action {
            Action::Move(dx, dy) => {
                let old = (self.player.x, self.player.y);
                self.player.try_move(dx, dy, &self.map);
                let newp = (self.player.x, self.player.y);
                if old != newp {
                    self.push_log(format!("Player moved to ({}, {})", newp.0, newp.1));
                }
                true
            }
            Action::Quit => false,
            Action::None => true,
        }
    }
}
