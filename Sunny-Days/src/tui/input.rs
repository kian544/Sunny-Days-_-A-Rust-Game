use crate::engine::action::Action;
use crossterm::event::{self, Event, KeyCode};

pub fn read_action() -> crossterm::Result<Action> {
    if let Event::Key(key) = event::read()? {
        let act = match key.code {
            KeyCode::Char('q') | KeyCode::Char('Q') => Action::Quit,

            KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('W') => Action::Move(0, -1),
            KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('S') => Action::Move(0, 1),
            KeyCode::Left | KeyCode::Char('a') | KeyCode::Char('A') => Action::Move(-1, 0),
            KeyCode::Right | KeyCode::Char('d') | KeyCode::Char('D') => Action::Move(1, 0),

            _ => Action::None,
        };
        Ok(act)
    } else {
        Ok(Action::None)
    }
}
