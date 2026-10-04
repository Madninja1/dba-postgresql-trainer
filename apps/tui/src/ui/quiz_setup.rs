use dba_trainer_domain::{QuestionLimit, QuizScope};

use crate::app::{App, QUESTION_LIMITS};
use ratatui::widgets::List;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    widgets::{Block, Borders, ListItem, Paragraph},
};

pub fn render(frame: &mut Frame, app: &App) {
    let areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let title = match app.quiz_scope {
        QuizScope::AllTopics => String::from("Общий тест"),

        QuizScope::Topic(topic_id) => app
            .topics
            .iter()
            .find(|topic| topic.id == topic_id)
            .map(|topic| format!("Тема: {}", topic.title))
            .unwrap_or_else(|| String::from("Тест по теме")),
    };

    let header = Paragraph::new(title);

    let items = QUESTION_LIMITS
        .iter()
        .enumerate()
        .map(|(index, limit)| {
            let style = if index == app.limit_selected {
                Style::default().add_modifier(Modifier::REVERSED)
            } else {
                Style::default()
            };

            ListItem::new(limit_label(*limit)).style(style)
        })
        .collect::<Vec<_>>();

    let list = List::new(items).block(Block::default().borders(Borders::ALL));

    let footer = Paragraph::new("↑/↓ или j/k — выбор | Enter — открыть | q — выход")
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(header, areas[0]);
    frame.render_widget(list, areas[1]);
    frame.render_widget(footer, areas[2]);
}

fn limit_label(limit: QuestionLimit) -> &'static str {
    match limit {
        QuestionLimit::Twenty => "20 вопросов",

        QuestionLimit::Fifty => "50 вопросов",

        QuestionLimit::All => "Все вопросы",
    }
}
