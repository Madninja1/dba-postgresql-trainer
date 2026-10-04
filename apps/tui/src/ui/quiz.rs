use dba_trainer_domain::{QuestionLimit, QuizScope};

use ratatui::Frame;

use crate::app::App;

use super::common::render_message;

pub fn render(frame: &mut Frame, app: &App) {
    let scope = match app.quiz_scope {
        QuizScope::AllTopics => "Общий тест",

        QuizScope::Topic(_) => "Тест по тема",
    };

    let limit = match app.selected_limit() {
        QuestionLimit::Twenty => "20",
        QuestionLimit::Fifty => "50",
        QuestionLimit::All => "все",
    };

    let message = format!(
        "{scope}\nКоличество вопросов: {limit}\n\n\
        Сессия пока не подключена."
    );

    render_message(frame, "Тест", &message, "Esc - в главное меню | q - выход");
}
