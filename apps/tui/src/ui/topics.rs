use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

use crate::app::App;

use super::common::{render_message, topic_display_number};

pub fn render(frame: &mut Frame, app: &App) {
    let topics = app.topics_for_selected_course();

    if topics.is_empty() {
        render_message(
            frame,
            app.strings().topics_title,
            app.strings().topics_empty,
            app.strings().footer_back,
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

    let title = match app.selected_topic_course.as_deref() {
        Some(course_code) => format!(
            "{} → {}",
            app.strings().topics_title,
            course_code.to_uppercase(),
        ),
        None => app.strings().topics_title.to_string(),
    };

    let header = Paragraph::new(title)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    let items = topics
        .iter()
        .enumerate()
        .map(|(index, topic)| {
            let style = if index == app.topic_selected {
                Style::default().add_modifier(Modifier::REVERSED)
            } else {
                Style::default()
            };

            let display_number = topic_display_number(&topics, index);

            ListItem::new(format!("{display_number}. {}", topic.title)).style(style)
        })
        .collect::<Vec<_>>();

    let topics = List::new(items).block(Block::default().borders(Borders::ALL));

    let mut topics_state = ListState::default();
    topics_state.select(Some(app.topic_selected));

    let footer = Paragraph::new(app.strings().footer_topics)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(header, areas[0]);
    frame.render_stateful_widget(topics, areas[1], &mut topics_state);
    frame.render_widget(footer, areas[2]);
}
