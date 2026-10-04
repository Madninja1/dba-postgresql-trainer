use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use crate::app::App;

use super::common::render_message;

pub fn render(frame: &mut Frame, app: &App) {
    if app.topics.is_empty() {
        render_message(
            frame,
            "Темы DBA",
            "Темы пока не загружены",
            "Esc - назад | q - выход",
        );

        return;
    }

    let areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let header = Paragraph::new("Темы DBA")
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    let items = app
        .topics
        .iter()
        .enumerate()
        .map(|(index, topic)| {
            let style = if index == app.topic_selected {
                Style::default().add_modifier(Modifier::REVERSED)
            } else {
                Style::default()
            };

            ListItem::new(topic.title.as_str()).style(style)
        })
        .collect::<Vec<_>>();

    let topics = List::new(items).block(Block::default().borders(Borders::ALL));

    let footer = Paragraph::new("↑/↓ или j/k — выбор | Enter — открыть | q — выход")
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(header, areas[0]);
    frame.render_widget(topics, areas[1]);
    frame.render_widget(footer, areas[2]);
}
