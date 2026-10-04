mod common;
mod home;
mod quiz;
mod quiz_setup;
mod statistics;
mod topics;

use ratatui::Frame;

use crate::app::{App, Screen};

pub fn render(frame: &mut Frame, app: &App) {
    match app.screen {
        Screen::Home => {
            home::render(frame, app);
        }

        Screen::Topics => {
            topics::render(frame, app);
        }

        Screen::QuizSetup => {
            quiz_setup::render(frame, app);
        }

        Screen::Quiz => {
            quiz::render(frame, app);
        }

        Screen::Statistics => {
            statistics::render(frame);
        }
    }
}
