use dba_trainer_domain::QuestionLimit;

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

    let header = Paragraph::new("Количество вопросов")
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

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
