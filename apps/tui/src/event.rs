use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::app::Action;

pub fn read_action() -> io::Result<Option<Action>> {
    let event = event::read()?;

    match event {
        Event::Key(key) if key.kind == KeyEventKind::Press => Ok(action_from_key(key)),

        _ => Ok(None),
    }
}

fn action_from_key(key: KeyEvent) -> Option<Action> {
    match (key.code, key.modifiers) {
        (KeyCode::Char('c'), modifiers) if modifiers.contains(KeyModifiers::CONTROL) => {
            Some(Action::Quit)
        }

        (KeyCode::Char('q'), _)
        | (KeyCode::Char('Q'), _)
        | (KeyCode::Char('й'), _)
        | (KeyCode::Char('Й'), _) => Some(Action::Quit),

        (KeyCode::Up, _) | (KeyCode::Char('k'), _) => Some(Action::Up),

        (KeyCode::Down, _) | (KeyCode::Char('j'), _) => Some(Action::Down),

        (KeyCode::Enter, _) => Some(Action::Confirm),

        (KeyCode::Esc, _) => Some(Action::Back),

        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q_quits() {
        let key = KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE);

        assert_eq!(action_from_key(key), Some(Action::Quit));
    }

    #[test]
    fn ctrl_c_quits() {
        let key = KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL);

        assert_eq!(action_from_key(key), Some(Action::Quit));
    }

    #[test]
    fn escape_goes_back() {
        let key = KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE);

        assert_eq!(action_from_key(key), Some(Action::Back));
    }
}
