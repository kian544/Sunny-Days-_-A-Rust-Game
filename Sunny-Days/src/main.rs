mod engine;
mod map;
mod tui;

use engine::game_loop::run;

fn main() -> crossterm::Result<()> {
    run()
}
