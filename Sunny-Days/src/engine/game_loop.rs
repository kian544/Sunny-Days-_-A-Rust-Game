use crate::engine::action::Action;
use crate::engine::world::World;
use crate::tui::{input::read_action, renderer::render};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{backend::CrosstermBackend, Terminal};

use std::{
    io,
    time::{Duration, Instant},
};

const MOVE_COOLDOWN_MS: u64 = 90; // ~11 moves/sec max, feels roguelike-y

pub fn run() -> std::io::Result<()> {
    // ---- terminal init ----
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // ---- game init ----
    let seed = rand::random::<u64>();
    let mut world = World::new(seed, 80, 45);

    let tick_rate = Duration::from_millis(60);
    let mut last_move_time = Instant::now() - Duration::from_millis(MOVE_COOLDOWN_MS);

    // ---- main loop ----
    let mut running = true;
    while running {
        terminal.draw(|f| render(f, &world))?;

        if event::poll(tick_rate)? {
            let mut action = read_action()?;

            // Debounce movement so it doesn't depend on OS key-repeat speed
            if let Action::Move(_, _) = action {
                let now = Instant::now();
                if now.duration_since(last_move_time) < Duration::from_millis(MOVE_COOLDOWN_MS) {
                    action = Action::None; // ignore too-fast repeat
                } else {
                    last_move_time = now;
                }
            }

            running = world.apply_action(action);
        } else {
            running = world.apply_action(Action::None);
        }
    }

    // ---- restore terminal ----
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
