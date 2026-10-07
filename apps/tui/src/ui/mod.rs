mod cancel_session;
mod common;
mod courses;
mod feedback;
mod home;
mod quiz;
mod quiz_setup;
mod results;
mod resume;
mod statistics;
mod topic_courses;
mod topics;

use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    widgets::Paragraph,
};

use crate::app::{App, Screen};

pub fn render(frame: &mut Frame, app: &App) {
    match app.screen {
        Screen::Home => {
            home::render(frame, app);
        }

        Screen::ResumeSession => {
            resume::render(frame, app);
        }

        Screen::TopicCourses => {
            topic_courses::render(frame, app);
        }

        Screen::Topics => {
            topics::render(frame, app);
        }

        Screen::Courses => {
            courses::render(frame, app);
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

        Screen::CancelSession => {
            cancel_session::render(frame, app);
        }

        Screen::Results => {
            results::render(frame, app);
        }

        Screen::Statistics => {
            statistics::render(frame, app);
        }
    }

    render_error(frame, app);
}

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
        Paragraph::new(format!("{}: {error}", app.strings().error)).alignment(Alignment::Center),
        error_area,
    );
}
