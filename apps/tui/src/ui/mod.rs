mod common;
mod feedback;
mod home;
mod quiz;
mod quiz_setup;
mod results;
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

        Screen::Feedback => {
            feedback::render(frame, app);
        }

        Screen::Results => {
            results::render(frame, app);
        }

        Screen::Statistics => {
            statistics::render(frame);
        }
    }

    render_error(frame, app);
}

use ratatui::{
    layout::{Alignment, Rect},
    widgets::Paragraph,
};

fn render_error(frame: &mut Frame, app: &App) {
    let Some(error) = app.error_message.as_deref() else {
        return;
    };

    let area = frame.area();

    if area.height == 0 {
        return;
    }

    let error_area = Rect::new(
        area.x,
        area.y + area.height.saturating_sub(1),
        area.width,
        1,
    );

    frame.render_widget(
        Paragraph::new(format!("Ошибка: {error}")).alignment(Alignment::Center),
        error_area,
    );
}
