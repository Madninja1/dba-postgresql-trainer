use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::app::{Action, Screen};

pub fn read_action(screen: Screen) -> io::Result<Option<Action>> {
    let event = event::read()?;

    match event {
        Event::Key(key) if key.kind == KeyEventKind::Press => Ok(action_from_key(screen, key)),
        _ => Ok(None),
    }
}

fn action_from_key(screen: Screen, key: KeyEvent) -> Option<Action> {
    if screen == Screen::PracticeEditor {
        return practice_editor_action(key);
    }

    match (key.code, key.modifiers) {
        (KeyCode::Char('c'), modifiers) if modifiers.contains(KeyModifiers::CONTROL) => {
            Some(Action::Quit)
        }
        (KeyCode::Char('q'), _)
        | (KeyCode::Char('Q'), _)
        | (KeyCode::Char('й'), _)
        | (KeyCode::Char('Й'), _) => Some(Action::Quit),
        (KeyCode::Char('c'), _) | (KeyCode::Char('с'), _) => Some(Action::ClearStatistics),
        (KeyCode::Char(' '), _) => Some(Action::Toggle),
        (KeyCode::Up, _) | (KeyCode::Char('k'), _) => Some(Action::Up),
        (KeyCode::Down, _) | (KeyCode::Char('j'), _) => Some(Action::Down),
        (KeyCode::Enter, _) => Some(Action::Confirm),
        (KeyCode::Esc, _) | (KeyCode::Backspace, _) => Some(Action::Back),
        _ => None,
    }
}

fn practice_editor_action(key: KeyEvent) -> Option<Action> {
    match (key.code, key.modifiers) {
        (KeyCode::Char('c'), modifiers) if modifiers.contains(KeyModifiers::CONTROL) => {
            Some(Action::Quit)
        }
        (KeyCode::Enter, modifiers) if modifiers.contains(KeyModifiers::CONTROL) => {
            Some(Action::PracticeCheck)
        }
        (KeyCode::F(5), _) => Some(Action::PracticeRun),
        (KeyCode::F(6), _) => Some(Action::PracticeReset),
        (KeyCode::F(7), _) => Some(Action::PracticeCheck),
        (KeyCode::Left, _) => Some(Action::EditorLeft),
        (KeyCode::Right, _) => Some(Action::EditorRight),
        (KeyCode::Up, _) => Some(Action::EditorUp),
        (KeyCode::Down, _) => Some(Action::EditorDown),
        (KeyCode::Esc, _) => Some(Action::Back),
        (KeyCode::Backspace, _) => Some(Action::EditorBackspace),
        (KeyCode::Enter, _) => Some(Action::EditorNewline),
        (KeyCode::Tab, _) => Some(Action::EditorTab),
        (KeyCode::Char(character), modifiers)
            if !modifiers.intersects(KeyModifiers::CONTROL | KeyModifiers::ALT) =>
        {
            Some(Action::EditorChar(character))
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q_quits_outside_editor() {
        let key = KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE);

        assert_eq!(action_from_key(Screen::Home, key), Some(Action::Quit));
    }

    #[test]
    fn q_is_text_inside_editor() {
        let key = KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE);

        assert_eq!(
            action_from_key(Screen::PracticeEditor, key),
            Some(Action::EditorChar('q'))
        );
    }

    #[test]
    fn ctrl_enter_checks_practice() {
        let key = KeyEvent::new(KeyCode::Enter, KeyModifiers::CONTROL);

        assert_eq!(
            action_from_key(Screen::PracticeEditor, key),
            Some(Action::PracticeCheck)
        );
    }

    #[test]
    fn f5_runs_practice() {
        let key = KeyEvent::new(KeyCode::F(5), KeyModifiers::NONE);

        assert_eq!(
            action_from_key(Screen::PracticeEditor, key),
            Some(Action::PracticeRun)
        );
    }

    #[test]
    fn escape_goes_back() {
        let key = KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE);

        assert_eq!(action_from_key(Screen::Home, key), Some(Action::Back));
    }
}
