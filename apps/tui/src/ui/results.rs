use ratatui::Frame;

use crate::app::App;

use super::common::render_message;

pub fn render(frame: &mut Frame, app: &App) {
    let percent = if app.answered_questions == 0 {
        0.0
    } else {
        (app.correct_answers as f64 / app.answered_questions as f64) * 100.0
    };

    let message = format!(
        "Правильных ответов: {} из {}\nРезультат: {:.0}%",
        app.correct_answers, app.answered_questions, percent,
    );

    render_message(
        frame,
        "Результат теста",
        &message,
        "Enter или Esc — главное меню | q — выход",
    );
}
