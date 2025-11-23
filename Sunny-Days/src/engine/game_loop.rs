use crate::engine::action::Action;
use crate::engine::world::World;
use crate::tui::{input::read_action, renderer::render};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{backend::CrosstermBackend, Terminal};

use std::{io, time::Duration};

pub fn run() -> std::io::Result<()> {

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let seed = rand::random::<u64>();
    let mut world = World::new(seed, 80, 45);

    let tick_rate = Duration::from_millis(60);

    let mut running = true;
    while running {
        terminal.draw(|f| render(f, &world))?;

        if event::poll(tick_rate)? {
            let action = read_action()?;
            running = world.apply_action(action);
        } else {
            running = world.apply_action(Action::None);
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
