use ratatui::Frame;

use crate::app::App;

use super::common::render_message;

pub fn render(frame: &mut Frame, app: &App) {
    let Some(stats) = app.statistics.as_ref() else {
        render_message(
            frame,
            "Статистика",
            "Статистика не загружена.",
            "Esc или Enter — назад | q — выход",
        );

        return;
    };

    let message = format!(
        "\
Завершено тестов: {}
Отменено тестов: {}

Отвечено вопросов: {}
Правильных ответов: {}
Неправильных ответов: {}

Точность: {:.1}%",
        stats.completed_sessions,
        stats.cancelled_sessions,
        stats.answered_questions,
        stats.correct_answers,
        stats.incorrect_answers(),
        stats.accuracy_percent(),
    );

    render_message(
        frame,
        "Статистика",
        &message,
        "c — очистить статистику | Esc или Enter — назад | q — выход",
    );
}
