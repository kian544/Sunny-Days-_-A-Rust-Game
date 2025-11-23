mod engine;
mod map;
mod tui;

use engine::game_loop::run;

fn main() -> std::io::Result<()> {
    run()
}

