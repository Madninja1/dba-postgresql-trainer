use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};

use crate::app::Action;

pub fn read_action() -> io::Result<Option<Action>> {
    let event = event::read()?;

    let action = match event {
        Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
            KeyCode::Up | KeyCode::Char('k') => Some(Action::Up),

            KeyCode::Down | KeyCode::Char('j') => Some(Action::Down),

            KeyCode::Enter => Some(Action::Confirm),

            KeyCode::Esc => Some(Action::Back),

            KeyCode::Char('q') => Some(Action::Quit),

            _ => None,
        },

        _ => None,
    };

    Ok(action)
}
