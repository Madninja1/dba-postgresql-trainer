use ratatui::Frame;

use crate::app::App;

use super::common::render_message;

pub fn render(frame: &mut Frame, app: &App) {
    let strings = app.strings();

    let percent = if app.answered_questions == 0 {
        0.0
    } else {
        (app.correct_answers as f64 / app.answered_questions as f64) * 100.0
    };

    let message = format!(
        "{}: {} / {}\n{}: {:.0}%",
        strings.correct_answers_count,
        app.correct_answers,
        app.answered_questions,
        strings.result,
        percent,
    );

    render_message(frame, strings.result_title, &message, strings.footer_result);
}
