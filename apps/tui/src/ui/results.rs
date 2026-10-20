use ratatui::Frame;

use crate::app::App;

use super::common::render_message;

pub fn render(frame: &mut Frame, app: &App) {
    let strings = app.strings();
    let limit = app.selected_limit();

    let percent = if app.answered_questions == 0 {
        0.0
    } else {
        (app.correct_answers as f64 / app.answered_questions as f64) * 100.0
    };

    let threshold = limit.pass_threshold_percent();
    let status = if limit.is_passed(app.correct_answers, app.answered_questions) {
        strings.passed
    } else {
        strings.not_passed
    };

    let message = format!(
        "{}: {} / {}\n{}: {:.0}%\n{}: {:.0}%\n{}: {}",
        strings.correct_answers_count,
        app.correct_answers,
        app.answered_questions,
        strings.result,
        percent,
        strings.pass_threshold,
        threshold,
        strings.pass_status,
        status,
    );

    render_message(frame, strings.result_title, &message, strings.footer_result);
}
